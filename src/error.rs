#[derive(Debug)]
pub enum MenuError {
    OledHttp(jsonrpc_client_http::Error),
    OledClient(jsonrpc_client_core::Error),
}

impl From<jsonrpc_client_http::Error> for MenuError {
    fn from(err: jsonrpc_client_http::Error) -> MenuError {
        MenuError::OledHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for MenuError {
    fn from(err: jsonrpc_client_core::Error) -> MenuError {
        MenuError::OledClient(err)
    }
}
