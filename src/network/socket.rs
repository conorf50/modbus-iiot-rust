use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;
use std::error;
//	===============================================================================================



// fn parse_ip_address ( address_string : &str ) -> Result< IpAddr, AddrParseError> {
    
//     let result  = IpAddr::from_str( address_string )?;
//     Ok(result)

// }


// fn parse_socket_address ( address_string : &str ) -> Result<SocketAddr, std::net::AddrParseError>
// {

//     /*
//         Use the built in error handling types and return those instead of using a String based return
//         The '?' operator is short for the try! macro, it basically attempts the operation and will return an 
//         Ok (positive result, the expected item) or an Error of some kind

//         Ok(result) will return the result of the SocketAddr::FromStr function to the calling function, whether it
//         is an SocketAddr or an Error
//     */
//     let result =  SocketAddr::from_str ( address_string )?;
//     Ok(result)        

// }


pub fn parse_network_address ( address_string : &str, default_port : u16 ) -> Result<SocketAddr, Box<dyn error::Error>>
{

            /*
                Create a valid Socket by first parsing the Ip address and then appending the 
                port number to it
            */

        // convert the ip to the socket_address representation of it. Try to parse the IP, if not
        // successful return an Error.
        let addr = address_string.parse::<IpAddr>()?;
        //Now create a Socket
         // create a socket. 
        let socket = SocketAddr::new(addr,  default_port);
        Ok(socket)
            
}



#[test]
fn test_parse_network_address ()
{
    let result_1  = parse_network_address ( "127.0.0.1", 502 );
    assert! (result_1.is_ok());

    // let result_2  = parse_network_address ( "127.0.0.1:504",502 );
    // assert! ( result_2.is_ok () );

    // let socket_1 : SocketAddr = result_2.unwrap ();
    // assert! ( socket_1.is_ipv4 () );
    // assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    // assert_eq! ( socket_1.port (), 504 );

    let result_3  = parse_network_address ( "127.0.300.1",502 );
    assert! ( result_3.is_err () );

    let result_4 : Result< SocketAddr, Box<dyn error::Error> > = parse_network_address ( "::1", 502 );
    assert! ( result_4.is_ok () );

    // let result_5 : Result< SocketAddr, Box<dyn error::Error>> = parse_network_address ( "[::1]:504", 502 );
    // assert! ( result_5.is_ok () );

    // let socket_2 : SocketAddr = result_5.unwrap ();
    // assert! ( socket_2.is_ipv6 () );
    // assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    // assert_eq! ( socket_2.port (), 504 );

    let result_6 : Result< SocketAddr, Box<dyn error::Error>> = parse_network_address ( "::111111", 502 );
    assert! ( result_6.is_err () );

    let result_7 : Result< SocketAddr, Box<dyn error::Error>> = parse_network_address ( "127.0.0.1", 0 );
    assert! ( result_7.is_err () );

    let result_8 : Result< SocketAddr, Box<dyn error::Error>> = parse_network_address ( "", 502 );
    assert! ( result_8.is_err () );
}



// #[test]
// fn test_parse_ip_address ()
// {
//     let result_1 : Result< IpAddr, AddrParseError > = parse_network_address ( "127.0.0.1" );
//     assert! ( result_1.is_ok () );

//     let ip_1 : IpAddr = result_1.unwrap ();
//     assert! ( ip_1.is_ipv4 () );
//     assert_eq! ( format!("{}", ip_1 ), "127.0.0.1" );

//     let result_2 : Result< IpAddr, AddrParseError > = parse_network_address ( "127.0.0.1111" );
//     assert! ( result_2.is_err () );

//     let result_3 : Result< IpAddr, AddrParseError > = parse_network_address ( "::1" );
//     assert! ( result_3.is_ok () );

//     let ip_2 : IpAddr = result_3.unwrap ();
//     assert! ( ip_2.is_ipv6 () );
//     assert_eq! ( format! ( "{}", ip_2 ), "::1" );

//     let result_4 : Result< IpAddr, AddrParseError > = parse_network_address ( "::111111" );
//     assert! ( result_4.is_err () );

//     let result_5 : Result< IpAddr, AddrParseError > = parse_ip_address ( "" );
//     assert! ( result_5.is_err () );
// }



// //	===============================================================================================

// #[test]
// fn test_parse_socket_address ()
// {
//     let result_1 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1:502" );
//     assert! ( result_1.is_ok () );
    
//     let socket_1 : SocketAddr = result_1.unwrap ();
//     assert! ( socket_1.is_ipv4 () );
//     assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
//     assert_eq! ( socket_1.port (), 502 );

//     let result_2 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1111:502" );
//     assert! ( result_2.is_err () );

//     let result_3 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1" );
//     assert! ( result_3.is_err () );

//     let result_4 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::1]:502" );
//     assert! ( result_4.is_ok () );

//     let socket_2 : SocketAddr = result_4.unwrap ();
//     assert! ( socket_2.is_ipv6 () );
//     assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
//     assert_eq! ( socket_2.port (), 502 );

//     let result_5 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::111111]:502" );
//     assert! ( result_5.is_err () );

//     let result_6 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::1]" );
//     assert! ( result_6.is_err () );

//     let result_7 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "" );
//     assert! ( result_7.is_err () );
// }


    
//  

