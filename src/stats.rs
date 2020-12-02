use std::env;

use jsonrpc_client_core::*;
use jsonrpc_client_http::HttpTransport;
use log::{debug, info};

use crate::error::MenuError;
use crate::structs::{CpuStatPercentages, LoadAverage, MemStat, Uptime};

// TODO: Replace unwraps with snafu errors

pub fn cpu_stats_percent() -> std::result::Result<CpuStatPercentages, MenuError> {
    debug!("Creating HTTP transport for stats client.");
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

pub fn load_average() -> std::result::Result<LoadAverage, MenuError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.load_average().call()?;
    let l: LoadAverage = serde_json::from_str(&response).unwrap();

    Ok(l)
}

pub fn mem_stats() -> std::result::Result<MemStat, MenuError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.mem_stats().call()?;
    let m: MemStat = serde_json::from_str(&response).unwrap();

    Ok(m)
}

pub fn uptime() -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.uptime().call()?;
    let u: Uptime = serde_json::from_str(&response).unwrap();
    let hours = (u.secs / 60).to_string();

    Ok(hours)
}

jsonrpc_client!(pub struct PeachStatsClient {
    /// JSON-RPC request to get measurement of current CPU statistics.
    pub fn cpu_stats_percent(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current load average statistics.
    pub fn load_average(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current memory statistics.
    pub fn mem_stats(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get system uptime.
    pub fn uptime(&mut self) -> RpcRequest<String>;
});
