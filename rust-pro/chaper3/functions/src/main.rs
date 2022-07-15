/*
 * @Author: 27
 * @LastEditors: 27
 * @Date: 2022-07-15 13:55:56
 * @LastEditTime: 2022-07-15 14:58:25
 * @FilePath: /Road-Rust/rust-pro/chaper3/functions/src/main.rs
 * @description: type some description
 */
fn main() {
    println!("Hello, world!");
    another_function();
    // check_int_float();
    loop_for_rev();
}

fn another_function() {
    println!("Another function.");
}

fn loop_for_rev() {
    for number in (2..8).rev() {
        println!("{}!", number);
    }
}

// fn check_int_float(cond bool) {
//     let a = if condition {
//         2
//     } else {
//         3.3
//     }
//     println!("adsfadfadf {}", a)
// }
