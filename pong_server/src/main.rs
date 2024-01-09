use std::{net::{TcpListener, TcpStream, IpAddr}, str::FromStr, os::unix::fs::PermissionsExt, time::Duration};
use std::io::{prelude::*, BufReader, Write};
use std::io;
use pong_lib::{TerminalOutput};


fn main() -> io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Insufficient number of arguments provided");
    }

    let ip_address = IpAddr::from_str(&args[1])
        .expect("Error parsing Ip Address")
        .to_string();
    println!("IP Address: {:?}", ip_address);
    let ip_address_and_port = format!("{}:3737", ip_address);

    let tcp_listener = TcpListener::bind(ip_address_and_port).unwrap();

    // connect players
    let mut player_one = tcp_listener.accept().unwrap().0;
    println!("player 1 tcp stream: {:?}", player_one);
    let mut player_two = tcp_listener.accept().unwrap().0;
    println!("player 2 tcp stream: {:?}", player_two);

    // set streams as non blocking
    //player_one.set_nonblocking(true).expect("set_nonblocking call failed");
    //player_two.set_nonblocking(true).expect("set_nonblocking call failed");

    // begin game logic and loop sending of data to client
	let mut user_buffer = String::new();
    let mut term = TerminalOutput::new(80,40);

    println!("Beginning game logic...");
    term.run_server(&mut player_one, &mut player_two)?;
    Ok(())
}
