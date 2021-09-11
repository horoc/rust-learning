// 常量和不可变变量的区别
// 1.常量不可用mut
// 2.声明常量使用const关键字，他的类型必须被标注
// 3.常量可以在任意作用域内声明，包括全局
// 4.常量可以绑定常量表达式，无法绑定倒函数调用结果或只能运行时才能计算出来的值
// 常量的命名规范：必须全部大写，下划线隔开
const MAX_POINTS: u32 = 10_000;


fn main() {
    // 变量的隐藏
    let x = 5;
    // x = x + 1; 这么做不行，不可变变量不可修改
    let x = x + 1; // 这么做可以，称为隐藏，相当于新创建一个变量, 这行以后用的都是这个新的x
    // 隐藏还有一个好处，就是可以转换类型
    let price = 100;
    let price = price.to_string();
    
    // let number = "100".parse().expect("parse error") 会报错，因为编译器无法确认类型
    let number: u32 = "100".parse().expect("parse error");
    
    // rust 有四种标量类型
    // 1. 整数（i8,u8,i16,u16...i128,u128,isize,usize)
    // 2. 浮点类型 (f32,f64)
    // 3. 布尔类型 (bool)
    // 4. 字符类型 (char, 单引号表示，一个占4个字节，用unicode）
    
    // 数值操作
    let sum = 1 + 2;
    let diff = 2.1 - 1.1;
    let prod = 4 * 0;
    let quo = 54 / 3;
    let rem = 54 % 3;
        
    // 复合类型
    // 1. tuple
    let tup: (i32, f32, u8) = (100, 1.1, 1);
    println!("i32:{}, f32:{}, u8:{}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);
    
    // 2. 数组
    let arr = [1, 2, 3, 4, 5];
    println!("first:{}, second:{}", arr[0], arr[1]);
    let arr_x: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_y = [2; 3]; // 这里相当于声明 arr_y = [2, 2, 2];
}
