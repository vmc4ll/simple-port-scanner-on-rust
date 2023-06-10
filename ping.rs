use std::net::Ipv4Addr;
use std::net::IcmpSocket;

fn main() -> std::io::Result<()> {
    let ip_address = "192.168.0.1".parse::<Ipv4Addr>()?;
    let socket = IcmpSocket::new()?;
    let seq_no = 0;

    let buffer: [u8; 32] = [0; 32];
    let packet = socket.echo_request(seq_no, buffer)?;

    socket.send_to(packet.into(), (ip_address.into(), 0))?;

    Ok(())
}
