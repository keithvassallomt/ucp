use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};
use std::error::Error;
use local_ip_address::local_ip;

pub const SERVICE_TYPE: &str = "_ucp._tcp.local.";

pub struct Discovery {
    daemon: ServiceDaemon,
}

impl Discovery {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let daemon = ServiceDaemon::new()?;
        Ok(Self { daemon })
    }

    pub fn register(&self, device_id: &str, port: u16) -> Result<(), Box<dyn Error>> {
        // Get the local IP address
        let ip = local_ip()?;
        
        // Hostname usually needs to be unique on the network, but we'll base it on device ID for now.
        // Format: device_id.local.
        let hostname = format!("{}.local.", device_id);
        
        // Properties can be used to send public key fingerprint or other metadata
        let properties = [("version", "0.1.0"), ("id", device_id)];

        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            device_id,
            &hostname,
            &ip.to_string(),
            port,
            &properties[..],
        )?;
        
        self.daemon.register(service_info)?;
        println!("Registered service: {} on {}:{}", device_id, ip, port);
        Ok(())
    }

    pub fn browse(&self) -> Result<mdns_sd::Receiver<ServiceEvent>, Box<dyn Error>> {
        let receiver = self.daemon.browse(SERVICE_TYPE)?;
        Ok(receiver)
    }
}
