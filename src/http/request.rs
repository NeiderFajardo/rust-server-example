use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}


impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {},
            Err(e) => return Err(e),

        }*/ 

        //let resquest = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        // Esta línea equivale a las dos formas anteriores de manejar errores
        let resquest = str::from_utf8(buf)?;
        unimplemented!();
    }
} 

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            // Como lo último que hay en las expresiones es el string se retorna
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {

}