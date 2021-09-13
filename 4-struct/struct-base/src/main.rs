struct User {
    name: String,
    status: bool,
    n: u64,
}

// 实现copy
#[derive(Copy, Clone)]
struct SimpleSt{
    n: u32,
//    name: String,
//    String没有实现copy，结构体能够实现copy的前提是所有属性都能copy
}

fn main() {
    
    // 如果结构体声明为mut，那么所有内容都是可变的
    let mut user = User {
        name: String::from("x"),
        status: true,
        n: 10,
    };
    user.name = String::from("y");
    println!("user name:{}", user.name);

    // 更新语法, 定义一部分字段，剩下的字段用另一个变量中的字段值复制
    let other_user = User {
        name: String::from("z"),
        .. user
    };
    println!("user name:{}, status:{}", other_user.name, other_user.status);

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("color 0:{}, 1:{}, 2:{}", black.0, black.1, black.2);

    // 定义：unit-like struct , 指的是没有任何字段的结构体
    struct EmptySt;
    
    // 结构体所有权能否转移，取决于是否实现Copy
    let tmp_user = user;
    // println!("user name:{}", user.name); 这里会报错，因为user值的所有权已经转移到tmp_user了
    let simple = SimpleSt {
        n: 1,
        // name: String::from("k"),
    };
    let tmp_simple = simple;
    println!("simple n:{}", simple.n); 
}
