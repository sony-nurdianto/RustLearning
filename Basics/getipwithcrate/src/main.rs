use get_if_addrs::get_if_addrs;

fn main() {
    match get_if_addrs() {
        Ok(interfaces) => {
            for iface in interfaces {
                println!("Interface: {}", iface.name);
                match iface.addr {
                    get_if_addrs::IfAddr::V4(addr) => {
                        if addr.is_loopback() {
                            println!("Localhost IPv4: {}", addr.ip);
                        } else {
                            println!("IPv4: {}", addr.ip);
                        }
                    }
                    get_if_addrs::IfAddr::V6(addr) => {
                        if addr.is_loopback() {
                            println!("Localhost IPv6: {}", addr.ip);
                        } else {
                            println!("IPv6: {}", addr.ip);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
