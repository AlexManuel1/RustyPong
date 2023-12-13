mod pong;
use pong::TerminalOutput;
use std::io;

use std::net::IpAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?} , Args Length: {}", args, args.len());

    if args.len() == 3 && args[1] == "-c" { // IP address is given, connect to server
        // parse IP address, return error if not parsed correctly
        connect_to_server_and_run_game(&args[2]).await;
    } else if args.len() == 2 && args[1] == "-s" {
        // create server and wait for another player to join game 
        println!("Creating Server...");
        create_server_and_run_game().await;
    } 
    else {
        // create local game of pong
    }

    let mut term = TerminalOutput::new(160, 40);
    //term.run()?;
    Ok(())
} 

async fn create_server_and_run_game() {
    // creator of server sets terminal width/height for the game
}

async fn connect_to_server_and_run_game(ip_address: &str) {
    let ip_address = IpAddr::from_str(ip_address)
        .expect("Failed to parse IP Address");
    println!("Connecting to IP Address: {}", ip_address);
}