use std::fmt;

/*
These are custom error types that can be referenced elsewhere in the codebase

*/
#[derive(Debug)]
pub struct NetParseErr();
impl fmt::Display for NetParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self)
    }
}
impl std::error::Error for NetParseErr{}

// another custom error for a zero port
#[derive(Debug)]
pub struct ZeroPortErr();
impl fmt::Display for ZeroPortErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Port number cannot be zero: {}", self)
    }
}
impl std::error::Error for ZeroPortErr{}
