use core::errortypes::ModbusTelegramError;
use core::modbustelegram::ModbusTelegram;
use std::io::{Read, Write};
use std::net::TcpStream;

//	===============================================================================================

fn read_telegram_from_stream(
  stream: &mut TcpStream,
  expected_bytes: u8,
) -> Result<ModbusTelegram, ModbusTelegramError> {
  //let mut reply : Result< ModbusTelegram, String > = Err( "Tcp Read Failed".to_string () );

  let mut data = vec![0; expected_bytes as usize];
  let response = stream.read(&mut data);

  if response.is_ok() {
    let telegram = ModbusTelegram::new_from_bytes(&data);

    if telegram.is_ok() {
      return Result::Ok(telegram.unwrap());
    }
  }

  return Result::Err(ModbusTelegramError {
    message: "Could not create telegram".to_string(),
  });
}

//	===============================================================================================

fn write_telegram_to_stream(stream: &mut TcpStream, telegram: &ModbusTelegram) -> bool {
  // let mut reply: bool = false;

	let bytes = telegram.get_bytes();

  if bytes.is_ok(){

    let response = stream.write_all(&bytes.unwrap());

    if response.is_ok(){
      return true;
    }
    else{
      return false;
    }

  }
  else {
    // reply is already set to false so we assume that the telegram could not be written
    return false;
  }
}




//	===============================================================================================

pub fn process_modbus_telegram(stream: &mut TcpStream, telegram: &ModbusTelegram) -> Result<ModbusTelegram, ModbusTelegramError> {

    let write_telegram: &ModbusTelegram = telegram;
    let write_response: bool = write_telegram_to_stream(stream, write_telegram);

    // if the telegram wrote succesfully
    if write_response {
      let expected_bytes: Option<u8> = write_telegram.get_expected_byte_count();
	  
      let read_response = read_telegram_from_stream(stream, expected_bytes.unwrap())?;
        
      Ok(read_response)
    }
    else {
      return Result::Err(ModbusTelegramError{message: "Stream was empty.".to_string() } );
    }
}
