use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};

fn scan_ports(ip: &str, start: u16, end: u16) {
    let addr = ip.parse::<IpAddr>().expect("Invalid IP address");

    for port in start..=end {
        let addr_port = format!("{}:{}", addr, port);
        match TcpStream::connect_timeout(
            &addr_port.parse().unwrap(),
            std::time::Duration::from_secs(1),
        ) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => continue,
        }
    }
}

fn main() {
    print!("Enter IP address to scan: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ip = input.trim();
    let start_port = 1;
    let end_port = 1024;
    scan_ports(ip, start_port, end_port);
}
