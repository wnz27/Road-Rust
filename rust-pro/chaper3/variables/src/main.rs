/*
 * @Author: 27
 * @LastEditors: 27
 * @Date: 2022-07-14 16:52:40
 * @LastEditTime: 2022-07-14 17:49:31
 * @FilePath: /Road-Rust/rust-pro/chaper3/variables/src/main.rs
 * @description: type some description
 */

fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    println!("The value of s0 is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of s1 is: {}", spaces);
}
