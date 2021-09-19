fn main() {
    // 普通方式
    let v: Vec<i32> = Vec::new();

    // 使用宏的方式
    let v2 = vec![1,2,3];

    // 如果通过上下文可以推断出类型，就不用显示定义类型
    let mut v3 = Vec::new();
    v3.push(1);

    // 获取vec内容
    let x: &i32 = &v3[0];

    match v.get(2) {
        Some(element) => println!("the element is {}", element),
        None => println!("there is no element"),
    }

    // 遍历
    println!("reading...");
    let vec = vec![1,2,3];
    for i in &vec { 
        println!("{}", i); 
    }
    
    // 修改
    println!("modify...");
    let mut v4 = vec![1,2,3];
    for i in &mut v4 {
        *i += 1;
    }
    for i in &v4 { 
        println!("{}", i); 
    }

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("xx")),
        Cell::Float(1.1),
    ];
}

