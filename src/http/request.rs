use super::method::Method;
use std::convert::TryFrom;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};
use std::error::Error;
use std::str::{self, Utf8Error};
// use std::convert::From;
pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}


impl TryFrom<&[u8]> for Request{
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
       let request = str::from_utf8(buf)?;
       let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       let (path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       if protocol != "HTTP/1.1"{
        return Err(ParseError::InvalidProtocol);
       }
       unimplemented!();
    }

}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl From<Utf8Error> for ParseError{
    fn from(_ : Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}
impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "query: {}", &self.message())
    }

}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "query: {}", &self.message())
    }
}

impl Error for ParseError {

}

// tests 
#[test]
fn get_next_word_test(){
    let text = String::from("jeden dwa trzy");
    assert_eq!(get_next_word(&text[..]).unwrap(), (&text[..5], &text[6..] ));
    assert_eq!(get_next_word(&text[10..]), None);
}
//figure how to start separated function
#[test]
fn get_next_word_print_test(){
    let st = "GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...".to_string();
    let mut text= st.as_str();
    loop{
        match get_next_word(text){
            Some((w, t)) => {
                println!("{}", w);
                text = t;
                },
            None => break, 
       }
    }
}