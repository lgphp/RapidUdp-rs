use std::env;
use std::str::FromStr;



pub const UDP_WEB_ADDR :&str  =  "0.0.0.0:38100";
pub const REPORT_SLAVE_STATUS_API: &str = "/fastMaster/v1/reportStatus";
pub const GET_USING_UDPLIST_API: &str = "/fastMaster/v1/getUsingUdpList";
pub const REGISTER_UDP_INFO_API: &str = "/fastMaster/v1/registerUdp";


pub fn report_slave_status_api() -> String {
    let mut root_url = get_root_url();
    root_url.push_str(REPORT_SLAVE_STATUS_API);
    root_url.to_owned()
}

pub fn get_using_udp_api() -> String {
    let mut root_url = get_root_url();
    root_url.push_str(GET_USING_UDPLIST_API);
    root_url.to_owned()
}

pub fn register_udp_info_api() -> String {
    let mut root_url = get_root_url();
    root_url.push_str(REGISTER_UDP_INFO_API);
    root_url.to_owned()
}

fn get_root_url() -> String {
    if let Ok(root_url) = env::var("MASTER_ROOT_URL") {
        root_url
    } else {
        panic!("MASTER_ROOT_URL must be set!")
    }
}

pub fn local_host_ip() -> String {
    if let Ok(local_host_ip) = env::var("LOCAL_HOST_IP") {
        local_host_ip
    } else {
        panic!("local_host_ip must be set!")
    }
}

pub fn idcx() -> i8 {
    if let Ok( idcx) = env::var("LOCAL_IDCX") {
        let idc = i8::from_str(idcx.as_str()).unwrap();
        idc
    } else {
        panic!("local_host_ip must be set!")
    }
}


#[test]
fn test_report_slave_status_() {
    report_slave_status_api();
}




