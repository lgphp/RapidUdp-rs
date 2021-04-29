#![allow(dead_code, unused_imports, non_snake_case, unused_imports, unused_variables, unused_must_use)]
use tokio::time::Duration;
use crate::conf;
use log::{info, warn};
use serde::{Serialize , Deserialize};
#[derive(Debug , Serialize , Deserialize)]
pub struct ReportPacket{
    addr:String,
    state:i8,
    ip:String,
    data:AddrInfo,
}
#[derive(Debug , Serialize , Deserialize)]
pub struct AddrInfo{
    pub(crate) leftAddr:String,
    pub(crate) selfAddr:String,
    pub(crate) rightAddr:String
}


///
/// 上报udp状态
pub   fn report_udp_status(addr:String,state:i8, ip:String , info : AddrInfo){

    let postbody = ReportPacket{
        addr,
        state,
        ip,
        data: info
    };
    let agent = ureq::AgentBuilder::new().timeout_connect(Duration::from_secs(4))
    .timeout_read(Duration::from_secs(5)).timeout_write(Duration::from_secs(5)).build();
     let response = agent.post(conf::report_slave_status_api().as_str()).send_json(ureq::json!(postbody));
     if let Ok(res) = response {
         if res.status()==200 {
             info!("report udpserver  status OK! status: {}", res.status());
         }else{
             warn!("report udpserver  status Failed! status: {}", res.status())
         }
     }else{
         warn!("report udpserver status Failed!,timeout! but udpserver still will  start")
     }
}

///
/// 获取正在使用的udpserver , 请求这个api后，master 会将master自身挂掉前的udpserver重新发送到udp 并启动
///
pub fn get_using_udp(ip: String)  {
    let agent = ureq::AgentBuilder::new().timeout_connect(Duration::from_secs(4))
        .timeout_read(Duration::from_secs(5)).timeout_write(Duration::from_secs(5)).build();
    let response = agent.post(conf::get_using_udp_api().as_str()).send_json(ureq::json!({
        "ip":ip
    }));
    if let Ok(res) = response {
        if res.status()==200 {
            info!("get using udp OK! status: {}", res.status());
        }else{
            warn!("get using udp Failed! status: {}", res.status())
        }
    }else{
        warn!("get using udp list  Failed!,timeout!")
    }

}


pub fn register_self_to_master(idcx:i8 , ip:String){
    let agent = ureq::AgentBuilder::new().timeout_connect(Duration::from_secs(4))
        .timeout_read(Duration::from_secs(5)).timeout_write(Duration::from_secs(5)).build();

    let response = agent.post(conf::register_udp_info_api().as_str()).send_json(ureq::json!({
        "idcx": idcx ,
        "ip" : ip
    }));

    if let Ok(res) = response {
        if res.status()==200 {
            info!("register self to master  OK! status: {}", res.status());
        }else{
            warn!("register self to master  Failed! status: {}", res.status())
        }
    }else{
        warn!("register self to master  Failed!,timeout!")
    }

}

