use std::process::Command;
use std::{time::{Duration},thread,net::{SocketAddr,UdpSocket}};

pub mod hardware;
pub mod network;
pub mod node;

fn main() {
    // Define the network mask or subnet
    let network_mask = "10.0.1.1/24"; 
    let broadcast_address = "255.255.255.255";
    let broadcast_port: u16 = 33333; // Choose a port for broadcast

    // Create UDP socket for broadcasting
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to socket");
    socket.set_broadcast(true).expect("Failed to set broadcast option");

    // Create thread for periodic broadcast
    thread::spawn(move || {
        loop {
            let broadcast_message = "RUSTY_NODE"; // Replace with your application identifier
            socket.send_to(broadcast_message.as_bytes(), &SocketAddr::new(broadcast_address.parse().expect("Invalid config IP"), broadcast_port))
                .expect("Couldn't send broadcast");
            thread::sleep(Duration::from_secs(5)); // Broadcast every 5 seconds (adjust as needed)
        }
    });

    println!("nmap -sU -p {} {}",broadcast_port,network_mask);
    // Construct the nmap command
    let output = Command::new("nmap")
        .arg(format!("-sU -p {} {}",broadcast_port, network_mask)) // Scan for hosts only (ping scan)
        .output()
        .expect("Failed to execute nmap command");


        println!("Nmap! done");
    let mut devices = vec![];

    // Check if the command executed successfully
    if output.status.success() {
        // Parse the nmap output (you might need to adjust this based on nmap's output format)
        let output_str = String::from_utf8_lossy(&output.stdout); 
        println!("Nmap output:\n{}", output_str); 

        // Extract IP addresses from the output (this is a simplified example)
        // You'll likely need more robust parsing logic
        for line in output_str.lines() {
            if line.contains("Nmap scan report for ") {
                let ip = line.split_whitespace().nth(4).unwrap(); 
                devices.push(ip);
            }
        }
    } else {
        println!("nmap command failed: {:?}", output.stderr);
    }
}