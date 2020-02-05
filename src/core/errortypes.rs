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
impl std::error::Error for NetParseErr {}

impl From<std::io::Error> for NetParseErr {

  // '_' gets rid of the "unused variable" warning 
  fn from(_ : std::io::Error) -> Self {
    NetParseErr {}
  }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

// another custom error for a zero port
#[derive(Debug)]
pub struct ZeroPortErr();
impl fmt::Display for ZeroPortErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Port number cannot be zero (port specified was {})", self)
  }
}
impl std::error::Error for ZeroPortErr {}

impl From<std::io::Error> for ZeroPortErr {
  fn from(_: std::io::Error) -> Self {
    ZeroPortErr {}
  }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

// Error for a coil
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct CoilError {
  pub  message: String,
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for CoilError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for CoilError {}

impl From<std::io::Error> for CoilError {
  fn from(error: std::io::Error) -> Self {
    CoilError {
      message: error.to_string(),
    }
  }
}
/////////////////////////////////////////////////////////////////////////////////////////////////

// ModbusTelegramError
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct ModbusTelegramError {
  pub message: String,
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for ModbusTelegramError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}
impl std::error::Error for ModbusTelegramError {}

impl From<std::io::Error> for ModbusTelegramError {
  fn from(error: std::io::Error) -> Self {
    ModbusTelegramError {
      message: error.to_string(),
    }
  }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

// Generic error for a register
// Uses some examples from here: https://learning-rust.github.io/docs/e7.custom_error_types.html
#[derive(Debug)]
pub struct RegisterError {
  pub message: String,
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for RegisterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}
impl std::error::Error for RegisterError {}

impl From<std::io::Error> for RegisterError {
  fn from(error: std::io::Error) -> Self {
    RegisterError {
      message: error.to_string(),
    }
  }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

// For errors that occur during data conversion
#[derive(Debug)]
pub struct DataTransformError {
  pub message: String,
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for DataTransformError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error during data conversion (error:'{}')", self.message)
  }
}
impl std::error::Error for DataTransformError {}

impl From<std::io::Error> for DataTransformError {
  fn from(error: std::io::Error) -> Self {
    DataTransformError {
      message: error.to_string(),
    }
  }
}

// For errors that occur during function code parsing
#[derive(Debug)]
pub struct FunctionCodeError {
  pub message: String,
  pub code: u8,
}

// Using this, we can call the same error with different messages to display to the end-user
impl fmt::Display for FunctionCodeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Encountered error {} during parsing of function code {}",
      self.message, self.code
    )
  }
}
impl std::error::Error for FunctionCodeError {}

impl From<std::io::Error> for FunctionCodeError {
  fn from(error: std::io::Error) -> Self {
    FunctionCodeError {
      // TODO, fix this
      message: error.to_string(),
      code: 0,
    }
  }
}
