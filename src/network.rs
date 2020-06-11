extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::MenuError;
use crate::structs::Traffic;

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_ap` method.
///
pub fn network_activate_ap() -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_ap().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_client` method.
///
pub fn network_activate_client() -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_client().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ip` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
///
pub fn network_ip(iface: &str) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ip(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `rssi` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
///
pub fn network_rssi(iface: &str) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.rssi(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ssid` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
///
pub fn network_ssid(iface: &str) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ssid(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `state` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
///
pub fn network_state(iface: &str) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.state(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `traffic` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
///
pub fn network_traffic(iface: &str) -> std::result::Result<Traffic, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.traffic(iface).call()?;
    let t: Traffic = serde_json::from_str(&response).unwrap();

    Ok(t)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// Creates a JSON-RPC request to activate the access point (ap0).
    pub fn activate_ap(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to activate the wireless client (wlan0).
    pub fn activate_client(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the IP address for the given interface.
    pub fn ip(&mut self, iface: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the average signal strength for the given interface.
    pub fn rssi(&mut self, iface: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn ssid(&mut self, iface: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the state for the given interface.
    pub fn state(&mut self, iface: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the network traffic for the given interface.
    pub fn traffic(&mut self, iface: &str) -> RpcRequest<String>;
});
