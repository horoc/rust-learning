#[derive(Debug)]
enum State {
    California,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}    

fn main() {
    //Option[T]是一个在prelude中引入的一个枚举，包含Some[T] 和 None
    let some_number = Some(5);
    let null_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
//    let sum = x + y; 直接加是会报错的

    // match
    value_in_cents(Coin::Quarter(State::NewYork));

    // 判断None
    let k = plus_one(Some(10));
    let n = plus_one(None);
    println!("k:{:?}, n:{:?}", k, n);

    // 通配符
    transfer_number(10); 

    // if let 简单匹配, 不用穷举所有case，只匹配一个
    let value = Some(10);
    let value_none = None;
    check_none(value);
    check_none(value_none);
}

// ！！在match中，必须穷举所有可能，不然编译会报错，这就很好地限制了npe
// 但是可以用_通配符绕过
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}

// 用if let
fn check_none(x: Option<i32>) {
    if let None = x {
        println!("this value is none");
    } else {
        println!("this value is not none");
    }
}

fn transfer_number(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("others"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state:{:?}", state);
            25
        },
    }
}
