fn main() {

    // if
    let number = 3;
    if number < 5 {
        println!("small than 5");
    } else if number > 5 {
        println!("greater than 5");
    } else {
        println!("equals to 5");
    }

    // if 本身是一个表达式
    let condition = true;
    let result = if condition { "true" } else { "false" };
    println!("the result is :{}", result);

    // loop 死循环
    let mut i = 0;
    let loop_result = loop {
        i += 1; 
        if i == 100 {
            break i;
        }
    };
   println!("loop result: {}", loop_result);

   // while
   i = 0;
   while i != 100 {
        i += 100;       
   }

   // for
   let arr = [1, 2, 3, 4, 5];
   for element in arr.iter() {
        println!("arr value :{}", element);
   }
   for element in 1..5 {
        println!("arr value :{}", element);
   }
   for element in (1..5).rev() {
        println!("arr value :{}", element);
   }
}
