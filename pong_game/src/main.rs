use pong_lib::TerminalOutput;
use std::io;

//use std::net::IpAddr;
//use std::str::FromStr;

fn main() -> io::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?} , Args Length: {}", args, args.len());
    // run game 
    let mut term = TerminalOutput::new(160, 40);
    term.run()?;
    Ok(())
} 
