use crate::http::method;
use super::{QueryString, Value as QueryValue};
use super::method::{Method, MethodError};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};
use std::error::Error;
use std::str::{self, Utf8Error};
//use std::cmp::PartialEq;
// use std::convert::From;
#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,//Option<&'buf str>,
    method: Method,
}

impl<'buf> Request<'buf>{
    pub fn path(&self)-> &str{
        &self.path
    }
    pub fn method(&self) -> &Method{
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }

}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
       let request = str::from_utf8(buf)?;
       let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       if protocol != "HTTP/1.1"{
        return Err(ParseError::InvalidProtocol);
       }
       
       let method:Method = method.parse()?;
       let mut query_strin = None;
       
       if let Some(i) = path.find('?'){
            query_strin = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
           
       }
       
       Ok(Self{
            path: path,
            query_string: query_strin,
            method: method,
        })
       
    }

}

impl<'buf> PartialEq for Request<'buf>{
    fn eq(&self, other: &Self) -> bool{
        self.path == other.path &&
        self.query_string.as_ref().unwrap().query_map == other.query_string.as_ref().unwrap().query_map &&
        self.method.to_string() == other.method.to_string()
    }
}
//TODO: remove and cleane code 
//*old function */
// fn get_next_word(request: &str) -> Option<(&str, &str)>{
//     for (i, c) in request.chars().enumerate(){
//         if c == ' ' || c == '\r'{
//             return Some((&request[..i], &request[i + 1..]));
//         }
//     }
//     None
// }
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    match request.find(|c| c==' ' || c=='\r'){
        Some(idx) =>{
            Some((&request[..idx], &request[idx +1..]))
        },
        None => None,
    }
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
impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
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
// TODO: remove or change this test
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
#[test]
fn test_try_from(){
    let st = "GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...".to_string();
    let buf = st.as_bytes();
    let map = HashMap::from([
                        ("name", QueryValue::Single("abc")),
                        ("sort", QueryValue::Single("1")),
    ]);
    assert_eq!(Request::try_from(buf).unwrap(), Request{path:"/search", query_string:Some(QueryString { query_map: map }), method:Method::GET});
    
}
