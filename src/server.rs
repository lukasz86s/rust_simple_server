use std::{net::TcpListener, io::Error};
use std::convert::TryFrom;
use crate::http::Request;
use std::io::Read;

pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self{
        Self{
            addr
        }
    }

    pub fn run<'a>(self) -> (){
        println!("{}", self.addr);
        println!("start servera");

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop{
            
            match listener.accept() {
                Ok((mut stream, _)) =>{
                    println!(" OK ");
                    let mut buffer = [0; 1024]; // create array with 1024 zeroes values
                    match stream.read(& mut buffer){
                        Ok(_) => {
                            println!("Recived a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]){
                                Ok(request) => {},
                                Err(e) =>{ println!("Failed to parse request: {}", e)},
                            }
                            
                        }
                        Err(e) => print!("Failed to read from connection: {}", e),
                    }
                    
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
            
            
        }

        
        
        

    }

}