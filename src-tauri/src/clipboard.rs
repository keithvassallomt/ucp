use crate::crypto;
use crate::protocol::Message;
use crate::state::AppState;
use crate::transport::Transport;
use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter};
use tauri_plugin_clipboard_manager::ClipboardExt;

// Use a shared cache to avoid feedback loops
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

static IGNORED_TEXT: Lazy<Arc<Mutex<Option<String>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// Read clipboard text using the Tauri clipboard plugin
fn read_clipboard(app: &AppHandle) -> Option<String> {
    app.clipboard().read_text().ok()
}

/// Write clipboard text using the Tauri clipboard plugin
fn write_clipboard(app: &AppHandle, text: &str) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

pub fn start_monitor(app_handle: AppHandle, state: AppState, transport: Transport) {
    thread::spawn(move || {
        let mut last_text = read_clipboard(&app_handle).unwrap_or_default();

        // Polling loop
        loop {
            // Check shutdown flag before each iteration
            if state.is_shutdown() {
                tracing::info!("Clipboard monitor received shutdown signal, exiting.");
                break;
            }

            if let Some(text) = read_clipboard(&app_handle) {
                // Check if this text should be ignored (because we just set it)
                let should_ignore = {
                    let ignored = IGNORED_TEXT.lock().unwrap();
                    if let Some(ref ign) = *ignored {
                        ign == &text
                    } else {
                        false
                    }
                };

                if should_ignore {
                    // Start tracking this as the new "baseline" so we don't trigger on it later
                    last_text = text;
                    // Clear the ignored text now that we've seen it
                    {
                        let mut ignored = IGNORED_TEXT.lock().unwrap();
                        *ignored = None;
                    }
                } else if text != last_text && !text.is_empty() {
                    tracing::debug!("Clipboard Changed detected (len={})", text.len());
                    last_text = text.clone();

                    // Update global deduplication cache to prevent echo loops
                    {
                        let mut last_global = state.last_clipboard_content.lock().unwrap();
                        *last_global = text.clone();
                    }

                    // Construct Payload Object
                    let hostname = crate::get_hostname_internal();
                    let msg_id = uuid::Uuid::new_v4().to_string();
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let payload_obj = crate::protocol::ClipboardPayload {
                        id: msg_id.clone(),
                        text: text.clone(),
                        timestamp: ts,
                        sender: hostname,
                    };

                    // Emit to frontend (local notification)
                    let _ = app_handle.emit("clipboard-change", &payload_obj);

                    // Check Auto-Send Setting
                    let auto_send = { state.settings.lock().unwrap().auto_send };
                    if !auto_send {
                        tracing::debug!("Auto-send disabled. Skipping broadcast.");
                    } else {
                        // Encrypt Payload using Cluster Key
                        let payload: Option<Vec<u8>> = {
                            let ck_lock = state.cluster_key.lock().unwrap();
                            if let Some(key) = ck_lock.as_ref() {
                                let mut key_arr = [0u8; 32];
                                if key.len() == 32 {
                                    key_arr.copy_from_slice(key);
                                    let json_payload =
                                        serde_json::to_vec(&payload_obj).unwrap_or_default();
                                    match crypto::encrypt(&key_arr, &json_payload) {
                                        Ok(cipher) => Some(cipher),
                                        Err(e) => {
                                            tracing::error!("Failed to encrypt clipboard: {}", e);
                                            None
                                        }
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        };

                        if let Some(encrypted_data) = payload {
                            let peers = state.get_peers();

                            if !peers.is_empty() {
                                let notifications =
                                    state.settings.lock().unwrap().notifications.clone();
                                if notifications.data_sent {
                                    crate::send_notification(
                                        &app_handle,
                                        "Clipboard Sent",
                                        "Clipboard content broadcasted to cluster.",
                                    );
                                }
                            }

                            let msg = Message::Clipboard(encrypted_data);
                            let data = serde_json::to_vec(&msg).unwrap_or_default();

                            for peer in peers.values() {
                                let target = peer.ip;
                                let port = peer.port;
                                let transport_clone = transport.clone();
                                let data_vec = data.to_vec();

                                tauri::async_runtime::spawn(async move {
                                    let addr = std::net::SocketAddr::new(target, port);
                                    if let Err(e) =
                                        transport_clone.send_message(addr, &data_vec).await
                                    {
                                        tracing::error!("Failed to send to {}: {}", addr, e);
                                    } else {
                                        tracing::info!("Sent clipboard to {}", addr);
                                    }
                                });
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
}

pub fn set_clipboard(app: &AppHandle, text: String) {
    let app_handle = app.clone();
    let text_clone = text.clone();

    thread::spawn(move || {
        // First check if clipboard already has this content to avoid unnecessary writes
        if let Some(current) = read_clipboard(&app_handle) {
            if current == text {
                tracing::debug!("Clipboard already contains this text, skipping write");
                return;
            }
        }

        // Mark this content as "to be ignored" by the monitor
        {
            let mut ignored = IGNORED_TEXT.lock().unwrap();
            *ignored = Some(text_clone);
        }

        if let Err(e) = write_clipboard(&app_handle, &text) {
            tracing::error!("Failed to set clipboard: {}", e);
        } else {
            tracing::debug!("Successfully set local clipboard content.");
        }
    });
}
