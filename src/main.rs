use std::net::UdpSocket;
use std::time::{Duration, Instant};

fn ping(ip: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket.set_read_timeout(Some(Duration::from_secs(1))).expect("Failed to set read timeout");

    let mut buf = [0u8; 1024];

    let local_addr = socket.local_addr().expect("Failed to get local address");
    println!("Local address: {}", local_addr);

    for _ in 0..3 {
        let start_time = Instant::now();
        socket.send_to(b"ping", &format!("{}:{}", ip, port)).expect("Failed to send data");

        match socket.recv_from(&mut buf) {
            Ok((_, addr)) => {
                let end_time = Instant::now();
                let rtt = end_time.duration_since(start_time).as_secs_f64() * 1000.0;
                println!("Ping reply from {}: time={:.2}ms", addr, rtt);
            },
            Err(_) => {
                println!("Ping request to {}:{} timed out", ip, port);
            }
        }
    }
}

fn scan_ports(ip: &str) {
    println!("Scanning ports for {}...", ip);

    let start_time = Instant::now();
    
    for port in 0..1024 {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        socket.set_read_timeout(Some(Duration::from_secs(1))).expect("Failed to set read timeout");

        let mut buf = [0u8; 1024];
        let addr = format!("{}:{}", ip, port);

        match socket.send_to(b"ping", &addr) {
            Ok(_) => {
                match socket.recv_from(&mut buf) {
                    Ok((_, _)) => println!("Port {}: Open", port),
                    Err(_) => println!("Port {}: Closed", port)
                }
            },
            Err(_) => {
                println!("Port {}: Closed", port);
            }
        }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64() * 1000.0;
    println!("Scanning completed in {:.2}ms", elapsed_time);
}

fn main() {
    let ip = "127.0.0.1";
    let port = 1024;
    ping(ip, port);
    scan_ports(ip);
}
