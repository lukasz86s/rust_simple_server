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
        write!(f,"{}", *self as u16)
    }

}

#[test]
fn test_status_code(){
    let mut status = StatusCode::Ok;
    
    assert_eq!("Ok", status.reason_phrase());

    status = StatusCode::BadRequest;
    assert_eq!("Bad Request", status.reason_phrase());

    status = StatusCode::NotFound;
    assert_eq!("Not Found", status.reason_phrase());


}