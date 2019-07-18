extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::MenuError;
use crate::structs::CpuStatPercentages;

pub fn cpu_stats_percent() -> std::result::Result<CpuStatPercentages, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.cpu_stats_percent().call()?;
    let c: CpuStatPercentages = serde_json::from_str(&response).unwrap();

    Ok(c)
}

jsonrpc_client!(pub struct PeachStatsClient {
    /// Creates a JSON-RPC request to get the IP address for the given interface.
    pub fn cpu_stats_percent(&mut self) -> RpcRequest<String>;
});
