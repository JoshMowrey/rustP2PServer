use std::net::{TcpListener, TcpStream};
use std::io::Error;

use connections::TcpConnection;
pub mod connections;

fn main() {
    println!("Hello, world!");
    let mut listener: TcpListener = match create_listener() {
        Ok(listener) => listener,
        Err(e) => {
            panic!("{e}");
        },
    };

    // should panic if local_addr cannot be unwraped
    println!("{0}", listener.local_addr().unwrap());

    handle_incoming(&mut listener);
}


fn create_listener() -> Result<TcpListener, Error> 
{
    match TcpListener::bind("127.0.0.1:8080")
    {
        Ok(listener) => return Ok(listener),
        Err(_) => {
            match TcpListener::bind("127.0.0.1:0") {
                Ok(listener) => return Ok(listener),
                Err(e) => return Err(e),
            }
        }
    };
}


fn handle_incoming(listener: &mut TcpListener) 
{
    let mut connections: &mut Vec<TcpConnection> = &mut vec!();
    for stream in listener.incoming() {
        let mut connection_list = &mut connections;
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("{e}");
                break;
            }
        };
        let _ = handle_connection(stream, &mut connection_list);
        println!("{0}", connection_list.iter().count());
    }
    
}

fn handle_connection(stream: TcpStream, connections: &mut Vec<TcpConnection>) -> Result<(), Error>
{
    let new_socket_addr: String = match stream.peer_addr() {
        Ok(address) => address.to_string(),
        Err(e) => return Err(e),
    };
    connections.push(TcpConnection{live: true, connection: stream});
    for connection in connections.iter_mut() {
        let mut ret: String = new_socket_addr.to_string();
        ret.push('\n');
        
        let _ = connection.write(&ret.as_bytes());
    }
    

    connections.retain(|x| x.live == true);
    Ok(())
}


