use std::net::SocketAddr;

pub enum ConnectionInfo {
    Server {
        addr: SocketAddr,
    },
    Client {
        id: u128,
        addr: SocketAddr,
        server: SocketAddr,
    },
}

impl ConnectionInfo {
    pub fn is_server(&self) -> bool {
        match &self {
            ConnectionInfo::Server { .. } => true,
            _ => false,
        }
    }

    pub fn is_client(&self) -> bool {
        match &self {
            ConnectionInfo::Client { .. } => true,
            _ => false,
        }
    }

    pub fn addr(&self) -> &SocketAddr {
        match &self {
            ConnectionInfo::Client { addr, .. } => addr,
            ConnectionInfo::Server { addr } => addr
        }
    }

    pub fn server_addr(&self) -> &SocketAddr {
        match &self {
            ConnectionInfo::Client { server, .. } => server,
            ConnectionInfo::Server { addr } => addr
        }
    }
}
