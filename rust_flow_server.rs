use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};

fn find_msg_len(buf:& [u8], msg_size:usize) -> Option<(usize,usize)> {
    let msg_len = &buf[0..msg_size]; 
    let total = msg_len.iter().try_fold((0,0), |(total,length),i | {match i {
        32 => Err((total,length + 1)),
        48...57 => Ok((total * 10 + (i - 48) as usize,(length + 1))),
        _=> panic!("Something got borked"),
    }
    });
    match total {
        Ok (_) => None,
        Err (total) => Some(total)
    }
}
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0 as u8; 1440];
    loop {
        let peek_val = stream.peek(&mut buf);
        match peek_val {
            Ok(size) => {
                let mut msg_sizes = find_msg_len(&buf,size).unwrap();
                let mut bufsize = msg_sizes.0 + msg_sizes.1;
                let mut buf = vec![0 as u8;bufsize];
                match stream.read_exact(&mut buf) {
                    Ok(()) => {
                        let logline = String::from_utf8(buf.to_vec());
                        match logline {
                            Ok(str) => {
                                println!("{}",str);
                            },
                            Err(_) => {
                                println!("Unable to convert log to string");
                            }
                        }
                    },
                    Err(_) => {
                        println!("Failed to read {} from buffer",bufsize);
                    }
                }
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        } 
    }
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:2514").unwrap();
    println!("Server listening on port 2514");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    println!("Dropping client"); 
    drop(listener);
}
