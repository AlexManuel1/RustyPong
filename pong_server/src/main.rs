use std::{net::{TcpListener, TcpStream, IpAddr}, str::FromStr};
use std::io::{prelude::*, BufReader};


fn main() {
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

    let player_connections: [TcpStream; 2] = [
        tcp_listener.accept().unwrap().0,
        tcp_listener.accept().unwrap().0
    ];

    //for i in 0..2 {
        //let player_connection = tcp_listener.accept();
        //match player_connection {
            //Ok((socket, ip_address))  => {
                //println!("Socket ({:?}) at IP: {:?}", socket, ip_address);
               //player_connections[i] = socket;
            //},
            //Err(e) => println!("couldn't connect to client: {:?}", e)
        //}
    //}

    println!("player connections: {:?}", player_connections);

    let [player_one, player_two] = player_connections;
    

    // incoming method on TcpListener returns an iterator and gives us a sequence of streams of type TcpStream.
    // a single stream represents an open connection. 
    // a connection is the name for a full req and res exchange AND the server closes the connection. 
    // We read from the TcpStream to see what the client sent and write our response to the stream to send data back to the client. 
    // We're actually iterating over connection attempts.
}

fn handle_clients(mut player1_stream: TcpStream, mut player2_stream: TcpStream) {
    let player1_buf_reader = BufReader::new(player1_stream);
    let player2_buf_reader = BufReader::new(player2_stream);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
