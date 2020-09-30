use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_num = rand::thread_rng().gen_range(1, 101);  // 生成1-100随机数，括号内左闭右开。利用rand的crate
    
    loop {
        println!("Please input your guess number!");

        let mut guess = String::new();  // 声明guess可变的变量

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");  // 异常处理，Result Type的变量，如果是Err则会输出给出的文本，可以见下面match重写用法
        
        let guess: u32 = match guess.trim().parse() {  // 覆盖guess，更改变量类型，声明mut并重写变量一般都是用于更改变量类型， u32，usigned 32bit int
            Ok(num) => num,
            Err(_) => continue,  // expect的方法的替代写法，更细粒度
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {  // match 使用，cmp， 比较guess猜测的数和生成的随机数。
            Ordering::Less => println!("Too small!!!"),  // 如果guess小于生成的数则提示猜的小了。
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }


}
