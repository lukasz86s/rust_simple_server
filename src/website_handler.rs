extern crate dunce;
use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;
use std::path::Path;


pub struct WebsiteHandler{
    public_path: String,
}
impl WebsiteHandler{
    pub fn new(public_path: String) -> Self{
        Self{public_path: public_path}
    }
    fn read_file(&self, file_path: &str) -> Option<String>{
        let path = format!("{}/{}", self.public_path, file_path);
        //secure to nobody read upon public dir data
        // TODO: test if canonicalize works on linux machine, have problem on windows
        match dunce::canonicalize(path) {
            Ok(path) =>{
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                }else{
                    print!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,  
        }
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method(){
            Method::GET => 
                match request.path(){
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                    path=> match self.read_file(path) {
                        Some(content) => Response::new(StatusCode::Ok, Some(content)),
                        None => Response::new(StatusCode::NotFound, None),  
                    } 
                },
            _ => Response::new(StatusCode::NotFound, None),

        }
        
    }
    
}