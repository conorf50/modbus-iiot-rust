use core::consts::*;
use core::errortypes::*;
use core::ethernet::*;
use core::methods::*;
use core::modbusreturn::*;
use core::modbustelegram::*;
use core::timehandling::*;
use network::common::create_tcp_stream;
use std::net::{Shutdown, TcpStream};
use std::result::Result;
use std::time::Duration;
use tcp::masteraccess::*;
use tcp::streamtelegram::*;
//	===============================================================================================

pub struct TcpClient {
  address: String,
  last_transaction_id: u16,
  port: u16,
  stream: Option<TcpStream>,
  unit_identifier: u8,
}

impl TcpClient {
  /// Creates a new `TcpClient` with the IPv4 or IPv6 address.
  /// The default MODBUS TCP port 502 while be used.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  ///
  /// use modbus_iiot::tcp::master::TcpClient;;
  ///
  /// //	with IPv4 address
  /// let mut client = TcpClient::new("127.0.0.1");
  /// //	or
  /// //	with IPv6 address
  /// let mut client = TcpClient::new("::1");
  /// ```
  pub fn new(address: &str) -> TcpClient {
    return Self::new_with_port(address, MODBUS_TCP_PORT);
  }

  /// Creates a new `TcpClient` with the IPv4 or IPv6 address
  /// and the specified TCP port.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  ///
  /// use modbus_iiot::tcp::master::TcpClient;;
  ///
  /// //	with IPv4 address
  /// let mut client = TcpClient::new_with_port("127.0.0.1", 504);
  /// //	or
  /// //	with IPv6 address
  /// let mut client = TcpClient::new_with_port("::1", 511);
  /// ```
  pub fn new_with_port(address: &str, port: u16) -> TcpClient {
    return Self::new_with_port_and_unitid(address, port, MODBUS_DEFAULT_UNIT_IDENTIFIER);
  }

  /// Creates a new `TcpClient` with the IPv4 or IPv6 address
  /// the specified TCP port and the unit id of the device.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  ///
  /// use modbus_iiot::tcp::master::TcpClient;;
  ///
  /// //	with IPv4 address
  /// let mut client = TcpClient::new_with_port_and_unitid("127.0.0.1", 504, 42);
  /// //	or
  /// //	with IPv6 address
  /// let mut client = TcpClient::new_with_port_and_unitid("::1", 511, 42);
  /// ```
  pub fn new_with_port_and_unitid(address: &str, port: u16, unit_id: u8) -> TcpClient {
    return TcpClient {
      address: address.to_string(),
      last_transaction_id: MODBUS_TRANSACTION_ID_INITIALIZER,
      port: port,
      stream: None,
      unit_identifier: unit_id,
    };
  }

  ///	Opens the connection to the device.
  /// If the connection is open the `Result` is Ok
  /// otherwise Err. Err than contais an error message.
  ///
  /// # Example
  ///
  /// ```rust,no_run
  ///
  /// use modbus_iiot::tcp::master::TcpClient;;
  ///
  /// let mut client = TcpClient::new("127.0.0.1");
  ///
  /// if let Err(message) = client.connect()
  /// {
  ///     println!("failure = {}", message);
  /// }
  /// else
  /// {
  ///     client.disconnect();    
  /// }
  /// ```

  pub fn connect(&mut self) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let connection = create_tcp_stream(&self.address, self.port)?;

    let timeout: Duration = Duration::from_millis(500);

    connection.set_read_timeout(Some(timeout))?;
    connection.set_write_timeout(Some(timeout))?;
    connection.set_nodelay(true)?;

    //self.stream = Some(connection);

    Ok(connection)
  }

  ///	Close the connection to the device if it is open.
  pub fn disconnect(&mut self) -> bool {
    let mut reply: bool = false;

    if self.stream.is_some() {
      if let Some(connection) = self.stream.take() {
        if let Ok(_) = connection.shutdown(Shutdown::Both) {
          reply = true;
        }
      }
    }

    return reply;
  }

  fn process_telegram(&mut self, request: &ModbusTelegram) -> Result<ModbusTelegram, ModbusTelegramError> {
    //let mut reply: ModbusTelegram;
    let stream = self.stream.take();
    
    if stream.is_some(){
      let reply = process_modbus_telegram(&mut stream.unwrap(), request)?;

      self.update_last_transaction_id();
      Ok(reply)
    }
    else {
      return Result::Err(ModbusTelegramError{message: "Stream was empty.".to_string() } );

    }

  }

  fn update_last_transaction_id(&mut self) {
    self.last_transaction_id = count_up_last_transaction_id(self.last_transaction_id);
  }
}

//	===============================================================================================

#[test]
fn test_count_up_last_transaction_id() {
  let test_data_1: u16 = 0x0001;
  let result_data_1: u16 = count_up_last_transaction_id(test_data_1);
  assert_eq!(result_data_1, 0x0002);

  let test_data_2: u16 = 0xFFFF;
  let result_data_2: u16 = count_up_last_transaction_id(test_data_2);
  assert_eq!(result_data_2, 0x0001);
}

fn count_up_last_transaction_id(last_transaction_id: u16) -> u16 {
  let reply: u16;

  if last_transaction_id == 0xFFFF {
    reply = MODBUS_TRANSACTION_ID_INITIALIZER;
  } else {
    reply = last_transaction_id + 1;
  }

  return reply;
}

//	===============================================================================================

impl EthernetMaster for TcpClient {
 
 
 
  fn read_coils(
    &mut self,
    starting_address: u16,
    quantity_of_coils: u16,
  ) -> Result<ModbusReturnCoils, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_read_coils(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      quantity_of_coils,
    )?;

    //if request_telegram.is_ok() {
    let request = self.process_telegram(&request_telegram)?;
	let response= self.process_telegram(&request)?;
	
    if verify_function_code(&request, &response) {
		let response_data = prepare_response_read_coils(&response.get_payload().unwrap(), quantity_of_coils);
		
		if response_data.is_ok(){
			
			return Result::Ok(process_response_of_coils(response_data.unwrap(), &start_time));
		}
		else {
			return Result::Err(ModbusTelegramError {
				message: response.get_function_code().unwrap().to_string(),
			  });
		}
		
	  } 
	  
	  else {
        return Result::Err(ModbusTelegramError {
          message: response.get_function_code().unwrap().to_string(),
        });
      }

  }

  fn read_discrete_inputs(&mut self, starting_address: u16, quantity_of_inputs: u16) -> Result<ModbusReturnCoils, ModbusTelegramError> {
    
	let start_time: Timestamp = Timestamp::new();
	
    let request_telegram = create_request_read_discrete_inputs(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      quantity_of_inputs,
    )?;

	let response = self.process_telegram(&request_telegram)?;
	
    if verify_function_code(&request_telegram, &response) {

        let response_data = prepare_response_read_discrete_inputs(&response.get_payload().unwrap(), quantity_of_inputs);
		if response_data.is_ok(){
			return Result::Ok(process_response_of_coils(response_data.unwrap(), &start_time));
		}
		else {
			return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
		}
	}

	else {
		return Result::Err(ModbusTelegramError{message: "Created modbus telgram is invalid.".to_string() } );
	}
  }

  fn read_holding_registers(&mut self, starting_address: u16, quantity_of_registers: u16) -> Result<ModbusReturnRegisters, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_read_holding_registers(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      quantity_of_registers,
    )?;
	
	let response = self.process_telegram(&request_telegram)?;

    if verify_function_code(&request_telegram, &response) {
		  
		let response_data = prepare_response_read_holding_registers(&response.get_payload().unwrap());
		if response_data.is_ok(){
			return Result::Ok (process_response_of_registers(response_data.unwrap(), &start_time));

		}
		else {
			return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
		}
	} 

     else {
		return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
    }

  }

  fn read_input_registers(&mut self, starting_address: u16, quantity_of_input_registers: u16) -> Result<ModbusReturnRegisters, ModbusTelegramError> {
    
    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_read_input_registers(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      quantity_of_input_registers,
	)?;

	  let response = self.process_telegram(&request_telegram)?; 
    
    

    if verify_function_code(&request_telegram, &response) {
		  
      let response_data = prepare_response_read_input_registers(&response.get_payload().unwrap());
      if response_data.is_ok(){
        return Result::Ok (process_response_of_registers(response_data.unwrap(), &start_time));
  
      }
      else {
        return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
      }
    } 
  
       else {
      return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
      }


  }

  fn write_single_coil(&mut self, output_address: u16, output_value: u16) -> Result<ModbusReturnCoils, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_write_single_coil(
      self.last_transaction_id,
      self.unit_identifier,
      output_address,
      output_value,
    )?;

      let response = self.process_telegram(&request_telegram)?;

        if verify_function_code(&request_telegram, &response) {
          let response_data = prepare_response_write_single_coil(&response.get_payload().unwrap());

          if response_data.is_ok(){
            return Result::Ok (process_response_of_coils(response_data.unwrap(), &start_time));
          }
          else {
            return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
          }
          //reply = process_response_of_coils(response_data, &start_time);
        }   
        else {
          return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
        }
  }

  fn write_single_register(&mut self, register_address: u16, register_value: u16) -> Result<ModbusReturnRegisters, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_write_single_register(
      self.last_transaction_id,
      self.unit_identifier,
      register_address,
      register_value,
    )?;

      let response = self.process_telegram(&request_telegram)?;

      if verify_function_code(&request_telegram, &response) {
          let response_data = prepare_response_write_single_register(&response.get_payload().unwrap());

          if response_data.is_ok(){
            return Result::Ok (process_response_of_registers(response_data.unwrap(), &start_time));
          }
          else {
            return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
          }

        } 
        else {
          // cannot verify request telegram
          return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
        }
  }

  fn write_multiple_coils( &mut self, starting_address: u16, quantity_of_outputs: u16, outputs_value: Vec<u8>,) -> Result<ModbusReturnRegisters, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_write_multiple_coils(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      quantity_of_outputs,
      outputs_value,
    )?;


      let response = self.process_telegram(&request_telegram)?;


      if verify_function_code(&request_telegram, &response) {
          let response_data = prepare_response_write_multiple_coils(&response.get_payload().unwrap());

          if response_data.is_ok(){
            return Result::Ok (process_response_of_registers(response_data.unwrap(), &start_time));
          }
          else {
            return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
          }
        }


      else {
        return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
      }
      
   

  }

  fn write_multiple_registers(&mut self, starting_address: u16, register_values: Vec<u16>) -> Result<ModbusReturnRegisters, ModbusTelegramError> {

    let start_time: Timestamp = Timestamp::new();
    let request_telegram = create_request_write_multiple_registers(
      self.last_transaction_id,
      self.unit_identifier,
      starting_address,
      register_values,
    )?;

      let response = self.process_telegram(&request_telegram)?;

        if verify_function_code(&request_telegram, &response) {
          let response_data = prepare_response_write_multiple_registers(&response.get_payload().unwrap());

          if response_data.is_ok(){
            return Result::Ok (process_response_of_registers(response_data.unwrap(), &start_time));
          }
          else {
            return Result::Err(ModbusTelegramError{message: "Could not verify response data was correct.".to_string() } );
          }

        }    
       else {
          return Result::Err(ModbusTelegramError{message: "Created modbus telegram was incorrect.".to_string() } );
        }
      }
}

//	===============================================================================================

#[test]
fn test_response_of_coils() {
  let timestamp: Timestamp = Timestamp::new();

  let test_data_1: Vec<bool> = vec![false, false, true, false, true, true, false, true];
  let result_1: ModbusReturnCoils = process_response_of_coils(test_data_1, &timestamp);
  assert!(result_1.is_good());

  let test_data_2: Vec<bool> = vec![];
  let result_2: ModbusReturnCoils = process_response_of_coils(test_data_2, &timestamp);
  assert!(result_2.is_bad());
}

fn process_response_of_coils(response_data: Vec<bool>, start_time: &Timestamp) -> ModbusReturnCoils {
  let reply: ModbusReturnCoils;

  if response_data.len() > 0 {
    reply = ModbusReturnCoils::Good(ReturnGood::new(response_data, start_time.elapsed_milliseconds()));
  } else {
    reply = ModbusReturnCoils::Bad(ReturnBad::new_with_message("modbus response data is invalid"));
  }

  return reply;
}

//	===============================================================================================

#[test]
fn test_process_response_of_registers() {
  let timestamp: Timestamp = Timestamp::new();

  let test_data_1: Vec<u16> = vec![0x000A, 0xFFFF, 0x00A8, 0xFF00];
  let result_1: ModbusReturnRegisters = process_response_of_registers(test_data_1, &timestamp);
  assert!(result_1.is_good());

  let test_data_2: Vec<u16> = vec![];
  let result_2: ModbusReturnRegisters = process_response_of_registers(test_data_2, &timestamp);
  assert!(result_2.is_bad());
}

fn process_response_of_registers(response_data: Vec<u16>, start_time: &Timestamp) -> ModbusReturnRegisters {
  let reply: ModbusReturnRegisters;

  //todo increase exception checking here (define max length)
  if response_data.len() > 0 {
    reply = ModbusReturnRegisters::Good(ReturnGood::new(response_data, start_time.elapsed_milliseconds()));
  } else {
    reply = ModbusReturnRegisters::Bad(ReturnBad::new_with_message("modbus response data is invalid"));
  }

  return reply;
}

//	===============================================================================================

impl MasterAccess for TcpClient {
  fn read_coils(&mut self, address: u16, quantity: u16) -> Vec<CoilValue> {
    let reply: Vec<CoilValue>;

    let response = EthernetMaster::read_coils(self, address, quantity);

    if response.is_ok() {
      
      reply = transform_modbus_return_coils(response.unwrap());
      //Ok(reply);

    } else {
      reply = vec![];
      //Ok (reply);
    }

   return reply;
  }

  fn read_discrete_inputs(&mut self, address: u16, quantity: u16) -> Vec<CoilValue> {
    let reply: Vec<CoilValue>;

    let response = EthernetMaster::read_discrete_inputs(self, address, quantity);

    if response.is_ok() {
      reply = transform_modbus_return_coils(response.unwrap());
    } else {
      reply = vec![];
    }

    return reply;
  }

  fn read_holding_registers(&mut self, address: u16, quantity: u16) -> Vec<u16> {
    let reply: Vec<u16>;

    let response = EthernetMaster::read_holding_registers(self, address, quantity);

    if response.is_ok() {
      reply = transform_modbus_return_registers(response.unwrap());
    } else {
      reply = vec![];
    }

    return reply;
  }

  fn read_input_registers(&mut self, address: u16, quantity: u16) -> Vec<u16> {
    let reply: Vec<u16>;

    let response = EthernetMaster::read_input_registers(self, address, quantity);

    if response.is_ok() {
      reply = transform_modbus_return_registers(response.unwrap());
    } else {
      reply = vec![];
    }

    return reply;
  }

  fn write_single_coil(&mut self, address: u16, value: CoilValue) -> bool {
    let mut reply: bool = false;

    let response = EthernetMaster::write_single_coil(self, address, convert_for_write_single_coil(&value));

    if response.is_ok() {
      reply = true;
    }

    return reply;
  }

  fn write_single_register(&mut self, address: u16, value: u16) -> bool {
    let response: ModbusReturnRegisters = EthernetMaster::write_single_register(self, address, value).unwrap();

    return response.is_good();
  }

  fn write_multiple_coils(&mut self, address: u16, coils: Vec<CoilValue>) -> bool {
    let mut reply: bool = false;

    if coils.len() > 0 {
      let values: Vec<u8> = transform_coils_to_bytearray(&coils);
      let response: ModbusReturnRegisters =
        EthernetMaster::write_multiple_coils(self, address, coils.len() as u16, values).unwrap();

      reply = response.is_good();
    }

    return reply;
  }

  fn write_multiple_registers(&mut self, address: u16, values: Vec<u16>) -> bool {
    let response: ModbusReturnRegisters = EthernetMaster::write_multiple_registers(self, address, values).unwrap();

    return response.is_good();
  }
}

//	===============================================================================================

#[test]
fn test_transform_modbus_return_coils() {
  let result_1: Vec<CoilValue> = transform_modbus_return_coils(ModbusReturnCoils::None);
  assert_eq!(result_1.len(), 0);

  let test_data_1: ReturnBad = ReturnBad::new_with_message("some error message");
  let result_2: Vec<CoilValue> = transform_modbus_return_coils(ModbusReturnCoils::Bad(test_data_1));
  assert_eq!(result_2.len(), 0);

  let test_data_2: ReturnGood<bool> = ReturnGood::new(vec![true, true, false, false, true, true, false, true], 1);
  let result_3: Vec<CoilValue> = transform_modbus_return_coils(ModbusReturnCoils::Good(test_data_2));
  assert_eq!(result_3.len(), 8);
}

fn transform_modbus_return_coils(returned_coils: ModbusReturnCoils) -> Vec<CoilValue> {
  let mut reply: Vec<CoilValue> = vec![];

  if returned_coils.is_good() {
    let values: Vec<bool> = returned_coils.unwrap_good().get_data();

    for coil in values {
      reply.push(CoilValue::set(coil));
    }
  }

  return reply;
}

//	===============================================================================================

#[test]
fn test_transform_modbus_return_registers() {
  let result_1: Vec<u16> = transform_modbus_return_registers(ModbusReturnRegisters::None);
  assert_eq!(result_1.len(), 0);

  let test_data_1: ReturnBad = ReturnBad::new_with_message("some error message");
  let result_2: Vec<u16> = transform_modbus_return_registers(ModbusReturnRegisters::Bad(test_data_1));
  assert_eq!(result_2.len(), 0);

  let test_data_2: ReturnGood<u16> = ReturnGood::new(vec![123, 456, 789], 1);
  let result_3: Vec<u16> = transform_modbus_return_registers(ModbusReturnRegisters::Good(test_data_2));
  assert_eq!(result_3.len(), 3);
}

fn transform_modbus_return_registers(returned_registers: ModbusReturnRegisters) -> Vec<u16> {
  let mut reply: Vec<u16> = vec![];

  if returned_registers.is_good() {
    reply = returned_registers.unwrap_good().get_data();
  }

  return reply;
}
