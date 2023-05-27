use std::{net::TcpListener, io::Error};
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
                            print!("Recived a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => print!("Failed to read from connection: {}", e),
                    }
                    
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
            
            
        }

        
        
        

    }

}