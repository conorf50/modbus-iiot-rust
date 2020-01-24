use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;

//	===============================================================================================

#[test]
fn test_parse_network_address ()
{
    let result_1 : Result< SocketAddr, AddrParseError > = parse_network_address ( "127.0.0.1", 502 );
    assert! ( result_1.is_ok () );

    let result_2 : Result< SocketAddr, AddrParseError > = parse_network_address ( "127.0.0.1:504",502 );
    assert! ( result_2.is_ok () );

    let socket_1 : SocketAddr = result_2.unwrap ();
    assert! ( socket_1.is_ipv4 () );
    assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    assert_eq! ( socket_1.port (), 504 );

    let result_3 : Result< SocketAddr, AddrParseError > = parse_network_address ( "127.0.300.1",502 );
    assert! ( result_3.is_err () );

    let result_4 : Result< SocketAddr, AddrParseError > = parse_network_address ( "::1", 502 );
    assert! ( result_4.is_ok () );

    let result_5 : Result< SocketAddr, AddrParseError > = parse_network_address ( "[::1]:504", 502 );
    assert! ( result_5.is_ok () );

    let socket_2 : SocketAddr = result_5.unwrap ();
    assert! ( socket_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    assert_eq! ( socket_2.port (), 504 );

    let result_6 : Result< SocketAddr, AddrParseError > = parse_network_address ( "::111111", 502 );
    assert! ( result_6.is_err () );

    let result_7 : Result< SocketAddr, AddrParseError > = parse_network_address ( "127.0.0.1", 0 );
    assert! ( result_7.is_err () );

    let result_8 : Result< SocketAddr, AddrParseError > = parse_network_address ( "", 502 );
    assert! ( result_8.is_err () );
}

pub fn parse_network_address ( address_string : &str, default_port : u16 ) -> Result<SocketAddr, std::net::AddrParseError>
{
// {
//     if address_string.is_empty () || default_port == 0x0000
//     {
//         Err( "address is empty or port is 0.".to_string ());
//     }
//     else
//     {

            let ip = parse_ip_address ( address_string );
            parse_socket_address ( address_string )?;
            Ok( SocketAddr::new ( ip.unwrap(),default_port ) )
            
   // }

}
//	===============================================================================================

#[test]
fn test_parse_ip_address ()
{
    let result_1 : Result< IpAddr, AddrParseError > = parse_ip_address ( "127.0.0.1" );
    assert! ( result_1.is_ok () );

    let ip_1 : IpAddr = result_1.unwrap ();
    assert! ( ip_1.is_ipv4 () );
    assert_eq! ( format!("{}", ip_1 ), "127.0.0.1" );

    let result_2 : Result< IpAddr, AddrParseError > = parse_ip_address ( "127.0.0.1111" );
    assert! ( result_2.is_err () );

    let result_3 : Result< IpAddr, AddrParseError > = parse_ip_address ( "::1" );
    assert! ( result_3.is_ok () );

    let ip_2 : IpAddr = result_3.unwrap ();
    assert! ( ip_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", ip_2 ), "::1" );

    let result_4 : Result< IpAddr, AddrParseError > = parse_ip_address ( "::111111" );
    assert! ( result_4.is_err () );

    let result_5 : Result< IpAddr, AddrParseError > = parse_ip_address ( "" );
    assert! ( result_5.is_err () );
}


fn parse_ip_address ( address_string : &str ) -> Result< IpAddr, AddrParseError> {
    //let reply : Result< IpAddr, String >;


    let result : Result< IpAddr, AddrParseError > = IpAddr::from_str( address_string );
    Ok(result.unwrap())
        
    //return reply;
}

//	===============================================================================================

#[test]
fn test_parse_socket_address ()
{
    let result_1 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1:502" );
    assert! ( result_1.is_ok () );
    
    let socket_1 : SocketAddr = result_1.unwrap ();
    assert! ( socket_1.is_ipv4 () );
    assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    assert_eq! ( socket_1.port (), 502 );

    let result_2 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1111:502" );
    assert! ( result_2.is_err () );

    let result_3 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "127.0.0.1" );
    assert! ( result_3.is_err () );

    let result_4 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::1]:502" );
    assert! ( result_4.is_ok () );

    let socket_2 : SocketAddr = result_4.unwrap ();
    assert! ( socket_2.is_ipv6 () );
    assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    assert_eq! ( socket_2.port (), 502 );

    let result_5 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::111111]:502" );
    assert! ( result_5.is_err () );

    let result_6 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "[::1]" );
    assert! ( result_6.is_err () );

    let result_7 : Result< SocketAddr, AddrParseError > = parse_socket_address ( "" );
    assert! ( result_7.is_err () );
}

fn parse_socket_address ( address_string : &str ) -> Result<SocketAddr, std::net::AddrParseError>
{

    /*
        Use the built in error handling types and return those instead of using a String based return
        The '?' operator is short for the try! macro, it basically attempts the operation and will return an 
        Ok (positive result, the expected item) or an Error of some kind

        Ok(result) will return the result of the SocketAddr::FromStr function to the calling function, whether it
        is an SocketAddr or an Error
    */
    let result =  SocketAddr::from_str ( address_string )?;
    Ok(result)        

}
    
   // return reply;

