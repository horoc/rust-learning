#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

#[derive(Debug)]
struct Ip {
    ipType: IpType,
    addr: String,
}

fn main() {
    let v4 = IpType::V4;
    let v6 = IpType::V6;

    let local = Ip {
        ipType: v4,
        addr: String::from("127.0.0.1"),
    };
    println!("{:?}", local);
    // println!("{:?}", v4); 已经被转移
    
}

fn route(ipType: IpType) {

}
