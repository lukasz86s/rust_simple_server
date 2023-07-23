use std::{net::TcpListener, io::Error};
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode, response};
use std::io::{Read, Write};

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

                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::NotFound,
                                         Some("<h1>IT'S WORKS!!!</h1>".to_string()))

                                    
                                },
                                Err(e) =>{ 
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                print!("failed to send response: {}", e);
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