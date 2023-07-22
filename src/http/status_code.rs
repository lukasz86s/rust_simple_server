use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Debug ,Clone , Copy)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode{
    pub fn reason_phrase(&self) -> &str{
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}
impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"Status: {}", *self as u16)
    }

}