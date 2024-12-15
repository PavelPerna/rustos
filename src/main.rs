use crate::udp::udp_server::server_start;

pub mod udp;

fn main() {
    let host = "10.0.1.15";
    let port = 6379;
    let _ = server_start(host, port);
}
