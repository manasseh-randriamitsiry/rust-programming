/*
The enum is used for a type having more than one possible value
*/

enum IpAddrKind {
    V4(),
    V6(),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {

}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let vfour = IpAddrKind::V4;
    let vsix = IpAddrKind::V6;
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
}
