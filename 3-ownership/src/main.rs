/*
 * 所有权规则：
 * 1. 每个值都有一个变量，这个变量是该值的所有者
 * 2. 每个值同时只能有一个所有者
 * 3. 当所有者超出作用域时，该值将会被删除
 */
fn main() {
    { 
        // line 10 之前，s不可用
        let s = "hello";
        // line 10 之后，s可用
    } 
    // println!("s:{}", s); line 12 之后，s不可用
    
    {
        let mut s = String::from("Hello");
        // 当s离开作用域后，rust自动调用drop函数，回收s指向堆内存的空间
    }
    
    // 变量移动
    {
        let s1 = String::from("s1");
        let s2 = s1; // line 23 以后，s1就不能使用了, 实际上这里是浅拷贝
        // println!("s1:{}", s1);
        // 为什么这么做？ 如果s1和s2都可用，那么drop时会重复释放一块空间两次

        // 深拷贝可以创建两个堆空间
        let s3 = String::from("s3");
        let s4 = s3.clone();
        // line 29后，s3和s4都能使用，因为对应的是不同的堆空间
        
        // copy trait
        // 如果一个类型实现了copy trait，那么旧的变量在新的变量赋值后仍然可以使用
        // 基本类型和基本类型的组合都实现了copy
        let x = 1;
        let y = x;
        // line 36 后， x和y是都可以使用的
        // 注意：如果一个类型或者该类型的一部分实现了drop trait，那么rust不允许他再实现copy trait
    }

    // 把值传给函数也会发生移动和复制
    let s = String::from("Hello World");
    take_ownership(s);
    // println!("string :{}", s); 无法通过编译
    
    let x = 1;
    take_ownership_i32(x); // 这里发生了复制
    println!("i32 :{}", x); // 这里能编译通过
    
    // 函数返回同样也会发生所有权的移动和复制
    let give_s = give_ownership();

    // 如何使用变量但是又不丢失所有权？
    // 第一种方式，使用返回值
    let cal_s = String::from("hello");
    let (cal_s, len) = cal_string_len(cal_s);
    println!("str:{}, len:{}", cal_s, len);
    // 第二种方式，使用引用, 引用不会改变变量的所有权
    let cal_ref = String::from("hello");
    let len_ref = cal_string_len_ref(&cal_ref);
    println!("str:{}, len:{}", cal_ref, len_ref);

    // 悬空引用, 引用了一块已经释放的空间, rust编译器保证不会出现这种引用
    // let r: &String = dangle(); rust编译不通过

    // 除了引用外，rust还有另外一种不持有所有权的数据类型，切片
    // 切片，指向数组一部分内容的引用
    let word = String::from("a b c");
    let sub_word = &word[1..2];
    let first_word = first_word(&word);
    println!("word:{}, first_word:{}", word, first_word);
}

fn take_ownership(s: String) {
    println!("string :{}", s);
}

fn take_ownership_i32(i: i32) {
    println!("i32 :{}", i);
}

fn give_ownership() -> String {
    let s = String::from("x");
    s
}

fn cal_string_len(cal_s: String) -> (String, usize) {
    let length = cal_s.len();
    (cal_s, length)
}

fn cal_string_len_ref(cal_s: &String) -> usize {
    // https://rust-book.junmajinlong.com/ch6/04_understand_mutable_ref.html
    // 可变引用与不可变引用的一篇好文
    
    // 引用默认不可变，但是如果需要可变，需要定义时为 &mut String
    // 1.但是可变引用有一个重要的特性，一个区域内可变引用不能超过1个
    // 2.对一个区域中，不可以对同一个变量同时存在可变引用和不可变引用
    cal_s.len() // 这里cal_s不会进行drop，因为引用不具备所有权
}

// fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//  }

// &str就是字符串切片的意思
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &value) in bytes.iter().enumerate(){
        if value == b' ' {
           return &s[..index];
        }
    }
    &s[..] 
}
