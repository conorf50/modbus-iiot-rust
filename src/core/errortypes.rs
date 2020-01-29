use std::fmt;

/*
These are custom error types that can be referenced elsewhere in the codebase

*/
#[derive(Debug)]
pub struct NetParseErr();
impl fmt::Display for NetParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse address: {}", self)
    }
}
impl std::error::Error for NetParseErr{}


/////////////////////////////////////////////////////////////////////////////////////////////////


// another custom error for a zero port
#[derive(Debug)]
pub struct ZeroPortErr();
impl fmt::Display for ZeroPortErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Port number cannot be zero (port specified was {})", self)
    }
}
impl std::error::Error for ZeroPortErr{}


/////////////////////////////////////////////////////////////////////////////////////////////////


// Error for a coil
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct CoilError{
    message: String
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for CoilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoilError{}

/////////////////////////////////////////////////////////////////////////////////////////////////


// ModbusTelegramError
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct ModbusTelegramError{
    message: String
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for ModbusTelegramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for ModbusTelegramError{}

/////////////////////////////////////////////////////////////////////////////////////////////////


// Generic error for a register
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct RegisterError{
    message: String
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for RegisterError{}


/////////////////////////////////////////////////////////////////////////////////////////////////

// For errors that occur during data conversion
#[derive(Debug)]
pub struct DataTransformError{
    message: String
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for DataTransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error during data conversion (error:'{}')", self.message)
    }
}
impl std::error::Error for DataTransformError{}