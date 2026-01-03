mod discovery;
mod peer;
mod state;
mod crypto;
mod transport;
mod clipboard;

use discovery::Discovery;
use state::AppState;
use peer::Peer;
use transport::Transport;
use tauri::{Manager, Emitter};
use rand::Rng;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_peers(state: tauri::State<AppState>) -> std::collections::HashMap<String, Peer> {
    state.get_peers()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .setup(|app| {
            // Initialize QUIC Transport on random port
            // Must be built within an async runtime context for Quinn
            let transport = tauri::async_runtime::block_on(async {
                Transport::new(0)
            }).expect("Failed to create transport");
            
            let port = transport.local_addr().expect("Failed to get port").port();
            println!("QUIC Transport listening on port {}", port);

            // Store transport in state? For now just keep it alive by moving to a leaked global or app state
            // But we need to keep it running.
            // For MVP, we can spawn the server accept loop here.
            
            let discovery = Discovery::new().expect("Failed to initialize discovery");
            
            // Generate a random device ID for this session
            let run_id: u32 = rand::rng().random();
            let device_id = format!("ucp-{}", run_id);

            // Register this device with the actual QUIC port
            discovery.register(&device_id, port).expect("Failed to register service");

            // Start browsing for peers
            let receiver = discovery.browse().expect("Failed to browse");
            
            let app_handle = app.handle().clone();
            let state_ref = app.state::<AppState>();
            let state_for_thread = (*state_ref).clone();

            tauri::async_runtime::spawn(async move {
                while let Ok(event) = receiver.recv_async().await {
                   match event {
                       mdns_sd::ServiceEvent::ServiceResolved(info) => {
                           if let Some(ip) = info.get_addresses().iter().next() {
                                let id = info.get_property_val_str("id").unwrap_or("unknown").to_string();
                                
                                let peer = Peer {
                                    id: id.clone(),
                                    ip: ip.to_string().parse().unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127,0,0,1))),
                                    port: info.get_port(),
                                    hostname: info.get_hostname().to_string(),
                                    last_seen: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                                };
                                
                                println!("Added Peer: {:?}", peer);
                                state_for_thread.add_peer(peer.clone());
                                let _ = app_handle.emit("peer-update", &peer);
                           }
                       }
                       _ => {}
                   }
                }
            });

            // Keep variables alive? Transport endpoint drops if not stored.
            app.manage(transport.clone());
            
            // Start Listening for incoming sync
            transport.start_listening(|data, addr| {
                 println!("Received {} bytes from {}", data.len(), addr);
                 if let Ok(text) = String::from_utf8(data) {
                     println!("Syncing remote clipboard content...");
                     clipboard::set_clipboard(text);
                 }
            });
            
            // Start Clipboard Monitor
            let transport_for_clipboard = transport.clone();
            // state_ref is borrowed from app. app is still valid here.
            // But we need AppState (inner)
            let state_for_clipboard = (*state_ref).clone(); 
            
            clipboard::start_monitor(app.handle().clone(), state_for_clipboard, transport_for_clipboard);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_peers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
