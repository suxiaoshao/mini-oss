use std::net::UdpSocket;

pub fn get_local_ip() -> Option<String> {
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
#[cfg(test)]
mod test {
    use crate::utils::get_local_ip::get_local_ip;

    #[test]
    fn test_local_ip() {
        println!("{}", get_local_ip().unwrap())
    }
}
