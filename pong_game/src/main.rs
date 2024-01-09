use pong_lib::TerminalOutput;
use std::io;
use std::io::*;

use std::net::{IpAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use std::process;

fn main() -> io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?} , Args Length: {}", args, args.len());

    let ip_address = IpAddr::from_str(&args[1])
        .expect("Error parsing Ip Address")
        .to_string();
    let port = &args[2];
    let ip_addr_and_port = format!("{}:{}", ip_address, port);

    println!("server (IP and port): {}", ip_addr_and_port);

    let mut stream = TcpStream::connect(ip_addr_and_port).unwrap();

    let mut term = TerminalOutput::new(80, 40);
    term.run_client(&mut stream)?;

    Ok(())
} 
