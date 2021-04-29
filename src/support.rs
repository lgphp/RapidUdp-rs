#![allow(dead_code, unused_imports , non_snake_case , unused_imports, unused_variables,unused_must_use)]
use  std::net::{IpAddr,SocketAddr};
pub fn parse_address(s: &str) -> IpAddr{
    let x:IpAddr = s.parse().unwrap();
    x
}





mod support {
    use crate::support::parse_address;
    use std::net::{IpAddr , SocketAddr,Ipv4Addr};
    use concurrent_hash_map::ConcurrentHashMap;
    #[test]
    pub fn test_parse_address() {
        let address:SocketAddr = "127.0.0.1:8000".parse().unwrap();
        // let x = match address.ip() {
        //      IpAddr::V4(a) => a ,
        //      _ => Ipv4Addr::new(127, 0, 0, 1)
        // };
        let ip : String  = format!("{:?}",address.ip());

        println!("{:?}" ,ip);

        let addr = String::from("127.0.0.1:8000");
        let x:Vec<&str> = addr.splitn(2, ":").collect();
        println!("{}" , x.first().unwrap());

    }

    #[test]
    pub fn  test_concurrent_map(){
       let mut c =  ConcurrentHashMap::new();
        c.insert("sss" , 3);
        for (k,v) in c.entries().iter(){
            println!("{}" , k)
        }

    }
}




