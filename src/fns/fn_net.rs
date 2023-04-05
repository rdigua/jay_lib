use std::net::UdpSocket;

///Get Local Ip Address.
/// 千万不要去用哪个local-ip, 或者machine ip之类的, 一个是通过ifconfig的返回获取IP地址的, 一个是扫描网卡, 在windows下都会panic
/// use jay_lib::fns::fn_net::get;
///
/// fn main() {
///     println!("{}", get().unwrap());
/// }
///
pub fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None,
    }
}