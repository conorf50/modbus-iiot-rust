use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;
use std::fmt;
//	===============================================================================================


// custom error handling

// see: https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxdyn-error

#[derive(Debug)]
struct NetParseErr();

impl fmt::Display for NetParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self)
    }
}

impl std::error::Error for NetParseErr{}


fn parse_ip_address ( address_string : &str ) -> Result< IpAddr, AddrParseError> {
    
    let result  = IpAddr::from_str( address_string )?;
    Ok(result)

}


pub fn parse_network_address ( address_string : &str, default_port : u16 ) -> Result<SocketAddr, Box<dyn std::error::Error>>
{
    /*
        Create a valid Socket by first parsing the Ip address and then appending the 
        port number to it
    */

    // return an instance of a custom Error type defined above

    if address_string.is_empty() {
        return Result::Err(Box::new(NetParseErr()));
    }
    else{    
        // convert the ip to the socket_address representation of it. Try to parse the IP, if not
        // successful return an Error.
        let addr = address_string.parse::<IpAddr>()?;
        // now create a socket. 
        let socket = SocketAddr::new(addr, default_port);
        Ok(socket)
    }   
}



#[test]
fn test_parse_network_address ()
{
    // Correct network address with a valid port
    let ipv4_1  = parse_network_address ( "127.0.0.1", 502 );
    assert! (ipv4_1.is_ok());

    let socket_1 : SocketAddr = ipv4_1.unwrap ();
    assert! ( socket_1.is_ipv4 () );
    assert_eq! ( format! ( "{}", socket_1.ip () ), "127.0.0.1" );
    assert_eq! ( socket_1.port (), 502 );


    // Local IPV4 address
    let ipv4_2  = parse_network_address ( "192.168.0.1",1502 );
    assert! ( ipv4_2.is_ok());

    let socket_2 : SocketAddr = ipv4_2.unwrap ();
    assert! ( socket_2.is_ipv4());
    assert_eq! ( format! ( "{}", socket_2.ip () ), "192.168.0.1" );
    assert_eq! ( socket_2.port (), 1502 );



    // Valid IpV6 address with valid port
    let ipv6_1 : Result< SocketAddr, Box<dyn Error>> = parse_network_address ( "::1", 502 );
    assert! ( ipv6_1.is_ok ());

    let socket_2 : SocketAddr = ipv6_1.unwrap ();
    assert! ( socket_2.is_ipv6());
    assert_eq! ( format! ( "{}", socket_2.ip () ), "::1" );
    assert_eq! ( socket_2.port (), 502 );

    // Valid IpV6 address with valid port
    // let ipv6_2 : Result<SocketAddr, Box<dyn error::Error>> = parse_network_address ( "fe80::a488:b9f7:398c:1745", 44234 );
    // assert! ( ipv6_2.is_ok());

    // let socket_3 : SocketAddr = ipv6_2.unwrap();
    // assert! ( socket_3.is_ipv6());
    // assert_eq! ( format! ( "{}", socket_3.ip () ), "fe80::a488:b9f7:398c:1745");
    // assert_eq! ( socket_3.port (), 44234);

    
    // Valid IpV6 address with valid port
    let ipv6_2 : Result< SocketAddr, Box<dyn Error> > = parse_network_address ( "::1", 502 );
    assert! ( ipv6_2.is_ok () );
    let socket_3 : SocketAddr = ipv6_2.unwrap();
    assert! ( socket_3.is_ipv6());
    assert_eq! ( format! ( "{}", socket_3.ip () ), "::1");
    assert_eq! ( socket_3.port (), 502);



    // IP address that's out of range
    let err_1  = parse_network_address ( "127.0.300.1",502 );
    assert! ( err_1.is_err ());
    
    // Incorrectly formatted ipv6 address
    let err_2 : Result< SocketAddr, Box<dyn Error>> = parse_network_address ( "::111111", 502 );
    assert! ( err_2.is_err () );

    // Passing a port of 0 should fail
    let err_3 : Result< SocketAddr, Box<dyn Error>> = parse_network_address ( "127.0.0.1", 0 );
    assert! (err_3.is_err());

    // Passing no address should fail
    let err_4 : Result< SocketAddr, Box<dyn Error>> = parse_network_address ( "", 502 );
    assert! ( err_4.is_err () );
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

