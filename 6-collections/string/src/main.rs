fn main() {
    let s = String::new();

    // to string
    let a = "some string";
    let b = a.to_string();

    // from
    let c = String::from("some string");
    
    // edit
    let mut d = String::from("x");
    d.push_str("y");
    let k = String::from("k");
    d.push_str(&k); 
    d.push('z');
    d += &k;
    println!("d = {}", d);

    // 加法操作
    let s0 = String::from("n");
    let s1 = String::from("m");
    // fn add(self, s: &str) -> String, 类似调用这样的函数, s0传给了self，所有权丢失,
    // s1传给了&str，所有权保留
    let s2 = s0 + &s1;     
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    // println!("s3 = {}", s3);
    
    // 使用format创建字符串
    let f = format!("{},{}", s1, s2);
    println!("f = {}", f);

    // String的本质：Vec<u8>
    
    // 长度
    let l = String::from("abcd").len();
    println!("len = {}", l);

    let data = String::from("abcd一二三四");

    // 打印字节
    for x in data.bytes() {
        println!("bytes: {}", x);
    }
    // unicode 标量 
    for x in data.chars() {
        println!("chars: {}", x);
    }

     // 切割string
    let hello = "hello";
    let sub = &hello[0..1];
    println!("sub = {}", sub);
}















