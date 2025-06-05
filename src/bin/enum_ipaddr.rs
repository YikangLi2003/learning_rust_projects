enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(self: &Self) {
        match self {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
            }
            IpAddr::V6(s) => {
                println!("IPv6 address: {}", s);
            }
        }
    }
}

fn main() {
    let addr_1 = IpAddr::V4(127, 0, 0, 1);
    let addr_2 = IpAddr::V4(127, 0, 0, 2);
    let addr_3 = IpAddr::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    let addr_4 = IpAddr::V6(String::from("2404:6800:4015:802::2004"));

    addr_1.print();
    addr_3.print();
    addr_2.print();
    addr_4.print();
}