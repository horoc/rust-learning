fn main() {
    println!("Hello, world!");
    foo_function();
    function_with_param(5);

    // 块表达式
    let y = {
        let x = 1;
        x + 3 // 这个块返回4, 注意，没有分号的语句成为表达式，表达式是又返回值的。
    };

    let result = add_five(6);
    println!("result:{}", result);
}

fn foo_function() {
    println!("foo function");
}

fn function_with_param(x: i32){
    println!("function with param x:{}", x);
}

fn add_five(x: i32) -> i32 {
    // rust里，函数返回值是函数体里最后一个表达式的值
    // 也可以使用return显示地返回值
    x + 5 //注意，这里没有分号，这是个表达式
}
