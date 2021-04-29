
use concurrent_hash_map::ConcurrentHashMap;
use std::collections::hash_map::RandomState;
use chrono::Local;
use std::net::SocketAddr;
use log::{info};
use std::net::UdpSocket;
use tokio::time::{ Duration};
use std::thread;

pub static mut UDP_LIST:Option<ConcurrentHashMap<String,i64, RandomState>> = None;

pub fn udp_monitor(){
    info!("udp 服务监控启动...");
    let socket = UdpSocket::bind("0.0.0.0:49999").unwrap();
    loop {
        unsafe {
            if let Some(ref mut m)= UDP_LIST{
                m.entries().iter().for_each(  |(k,v)| {
                    if Local::now().timestamp_millis() - v > 1000*20 {
                        let raddr:SocketAddr = k.to_string().parse().unwrap();
                        //发送一个指令使其退出
                        socket.connect(raddr).unwrap();
                        let _sent_size = socket.send("quit".as_bytes());
                        m.remove(k.to_string());
                    }
                })
            }
        }
        thread::sleep(Duration::from_millis(4000));
    }

}