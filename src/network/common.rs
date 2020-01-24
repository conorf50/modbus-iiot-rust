

use std::error;
use std::net::{TcpStream};
use network::socket::parse_network_address;

//	===============================================================================================

pub fn create_tcp_stream ( ip_address : &str, port : u16 ) -> Result<TcpStream, Box<dyn error::Error>>
{	
	let address_result = parse_network_address( ip_address,port)?;
	let connection_result = TcpStream::connect(address_result)?;
	Ok(connection_result)
}
