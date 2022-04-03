use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use rayon::prelude::*;

use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1234", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.is_empty() {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open)
        .collect();

    subdomain
}

fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = matches!(TcpStream::connect_timeout(&socket_address, timeout), Ok(_));

    Port { port, is_open }
}
