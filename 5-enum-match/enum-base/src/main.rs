#[derive(Debug)]
enum IpType {
    V4,
    V6,
}

// enum的每个变体中，可以有自己的值和类型
#[derive(Debug)]
enum IpTypeExtra {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举也能定义方法
impl IpTypeExtra {
    fn print_self(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Ip {
    ipType: IpType,
    ipTypeExtra: IpTypeExtra,
    addr: String,
}

fn main() {
    let v4 = IpType::V4;
    let v6 = IpType::V6;

    let local = Ip {
        ipType: v4,
        ipTypeExtra: IpTypeExtra::V4(127, 0, 0, 1),
        addr: String::from("127.0.0.1"),
    };
    println!("{:?}", local);
    // println!("{:?}", v4); 已经被转移

    IpTypeExtra::V6(String::from("127.0.0.1")).print_self();
}

