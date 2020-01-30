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

fn write_telegram_to_stream(stream: &mut TcpStream, telegram: &ModbusTelegram) -> Result<bool, ModbusTelegramError> {
  let mut reply: Result<bool, ModbusTelegramError> ;

	let bytes = telegram.get_bytes();
	if bytes.is_ok(){
		let response = stream.write_all(&bytes.unwrap());

		if response.is_ok() {
			// unwrap the success val from the reply
			Ok(reply.unwrap())
		  }

		else{
			return Result::Err(ModbusTelegramError{message: "Could not write some or all bytes to the stream.".to_string() } );
		}  
	}
	
	else{
		return Result::Err(ModbusTelegramError{message: "Could not get bytes from Modbus telegram.".to_string() } );
	} 
}

//	===============================================================================================

pub fn process_modbus_telegram(stream: &mut TcpStream, telegram: &ModbusTelegram) -> Result<ModbusTelegram, ModbusTelegramError> {
  let mut reply: ModbusTelegram;

    let write_telegram: &ModbusTelegram = telegram;
    let write_response: Result<bool, ModbusTelegramError> = write_telegram_to_stream(stream, write_telegram);
    let expected_bytes: Option<u8> = write_telegram.get_expected_byte_count();
	  
	let read_response = read_telegram_from_stream(stream, expected_bytes.unwrap())?;
    
	Ok(reply)
}
