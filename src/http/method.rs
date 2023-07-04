use std::{str::FromStr, string::ToString, string::ParseError};
//#[derive(Debug)]
#[derive(Debug)]
pub enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method{
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        match s{
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" =>Ok(Self::HEAD) ,
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }

    }
}
impl ToString for Method{
    fn to_string(&self) -> String{
        match self {
            Self::GET => "GET".to_string(),
            Self::DELETE => "DELETE".to_string(),
            Self::POST => "POST".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::HEAD => "HEAD".to_string(),
            Self::CONNECT => "CONNECT".to_string(),
            Self::OPTIONS => "OPTIONS".to_string(),
            Self::TRACE => "TRACE".to_string(),
            Self::PATCH => "PATCH".to_string(),
        }
    }
}
pub struct MethodError;



