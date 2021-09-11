
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// 一个结构体可以有多个impl块
impl Rectangle {
    // 定义方法
    // 方法的第一参数可以是：self, &self, mut self, &mut self
    fn cal_area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }

    // 关联函数（不是方法，类似于静态方法，比如String::from
    fn square(n: u32) -> Rectangle {
        Rectangle {
            width: n,
            length: n, 
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        length: 30,
    };
    println!("w:{}, l:{}, area:{}", rec.width, rec.length, cal_area(&rec));

    // {:?} 可以输出完整的结构体信息，但是需要结构体实现debug trait
    // {:#?} 也是打印完整信息，但输出的时候会格式化输出
    println!("{:?}", rec);
    println!("{:#?}", rec);

    // 使用结构体自己的方法计算
    println!("w:{}, l:{}, area:{}", rec.width, rec.length, rec.cal_area());

    // 是否能容纳另一个长方形
    println!("can hold ? {}", rec.can_hold(&(Rectangle{width: 10, length: 10})));

    // 关联函数
    println!("{:?}", Rectangle::square(15));
}

fn cal_area(rec: &Rectangle) -> u32 {
    rec.width * rec.length
}
