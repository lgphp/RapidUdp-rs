use tokio::net::{UdpSocket};
use std::net::{SocketAddr, AddrParseError};
use log::{info, warn};
use chrono::Local;
use crate::servermgr::UDP_LIST;
use crate::report::{report_udp_status, AddrInfo};


#[derive(Debug)]
pub struct Server {
    left_address: Option<SocketAddr>,
    raw_left_address: String,
    right_address: SocketAddr,
    raw_right_address: String,
    self_address: SocketAddr,
    raw_self_address: String,

}

#[derive(Debug)]
struct Conn {
    socket: UdpSocket,
    buf: Vec<u8>,
    received_bytes: Option<(usize, SocketAddr)>,
}

impl Server {
    pub fn new(left_address: String, self_address: String, right_address: String) -> Server {
        let mut left_addr: Option<SocketAddr> = None;
        let left_addr_str = left_address.as_str();
        if left_addr_str != "0.0.0.0:0000" {
            let result: Result<SocketAddr, AddrParseError> = left_addr_str.parse();
            if let Ok(l_addr) = result {
                left_addr = Some(l_addr);
            } else {
                panic!("左侧地址不合法!")
            }
        }
        let right_addr: SocketAddr = right_address.parse().expect("右侧地址不合法！");
        let self_addr: SocketAddr = self_address.parse().expect("自身地址不合法");
        let server = Server {
            left_address: left_addr,
            raw_left_address: left_address,
            right_address: right_addr,
            raw_right_address: right_address,
            self_address: self_addr,
            raw_self_address: self_address,
        };
        server
    }
    ///
    /// 启动本udp转发器
    pub async fn start(&mut self) -> std::io::Result<()> {
        let self_address_port = self.self_address.port();
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", self_address_port)).await?;
        info!("UDP 启动成功....{}", self.self_address);
        unsafe {
            if let Some(ref mut sm) = UDP_LIST {
                sm.insert(self.raw_self_address.clone(), Local::now().timestamp_millis());
            }
        }
        // 上报成功状态
        report_udp_status(self.raw_self_address.clone(), 2i8, self.self_address.ip().to_string(), AddrInfo {
            leftAddr: self.raw_left_address.clone(),
            selfAddr: self.raw_self_address.clone(),
            rightAddr: self.raw_right_address.clone(),
        });


        let conn = Conn {
            socket,
            buf: vec![0; 1024],
            received_bytes: None,
        };
        self.event_loop(conn).await?;
        Ok(())
    }


    ///
    /// 启动一个事件循环
    async fn event_loop(&mut self, conn: Conn) -> std::io::Result<()> {
        let Conn {
            socket,
            mut buf,
            mut received_bytes
        } = conn;
        loop {
            self.handler(&socket, &buf, received_bytes).await?;
            received_bytes = Some(socket.recv_from(&mut buf).await?);
            // 收到一个退出的数据包即可退出
            let b_size = received_bytes.unwrap().0 as u8;
            if b_size == 4 {
                if String::from_utf8_lossy(&buf[..4]).as_ref() == "quit" {
                    info!("{}===>退出", self.raw_self_address);
                    // 上报成功状态
                    report_udp_status(self.raw_self_address.clone(), 0i8, self.self_address.ip().to_string(), AddrInfo {
                        leftAddr: self.raw_left_address.clone(),
                        selfAddr: self.raw_self_address.clone(),
                        rightAddr: self.raw_right_address.clone(),
                    });

                    return Ok(());
                }
            }
        }
    }

    ///
    /// 处理udp数据
    ///
    async fn handler(&mut self, socket: &UdpSocket, buf: &Vec<u8>, received_bytes: Option<(usize, SocketAddr)>) -> std::io::Result<()> {
        if let Some((size, peer)) = received_bytes {
            if size > 0 {
                // 更新时间
                unsafe {
                    if let Some(ref mut sm) = UDP_LIST {
                        sm.insert(self.raw_self_address.clone(), Local::now().timestamp_millis());
                    }
                }

                // 右侧发来的消息转发给左侧
                if self.right_address.ip().eq(&peer.ip()) && peer.port() == self.right_address.port() {
                    if self.left_address.is_none() {
                        warn!("对端左侧地址还没有注册");
                        return Ok(());
                    }
                    let ret = socket.send_to(&buf[..size], self.left_address.unwrap()).await?;
                    info!("{} ### 收到来自右侧 {} ({}) ==> {} ", self.raw_self_address, self.raw_right_address, ret, self.raw_left_address)
                } else {
                    // 否则 ，一定是左侧地址发来的信息，转发给右侧
                    // 如果来自消息的Ip地址和分配的左侧地址不一样，证明不是中继服务器发来的，而是客户端发来的，需要替换左侧地址
                    //动态更新左侧地址，因为左侧地址会随着网络条件的变化而变化
                    // if  self.left_address.is_none(){
                    self.left_address = Some(peer);
                    self.raw_left_address = format!("{}:{}", peer.ip(), peer.port());
                    let ret = socket.send_to(&buf[..size], self.right_address).await?;
                    info!("{} ### 收到来自左侧 {}  ({})  ==> {} ", self.raw_self_address, self.raw_left_address, ret, self.raw_right_address)
                }
            }
        }

        Ok(())
    }
}

