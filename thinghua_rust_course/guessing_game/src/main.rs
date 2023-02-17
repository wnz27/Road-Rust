/*
 * @Author: 27
 * @LastEditors: 27
 * @Date: 2023-02-18 00:26:12
 * @LastEditTime: 2023-02-18 00:27:37
 * @FilePath: /Road-Rust/thinghua_rust_course/guessing_game/src/main.rs
 * @description: type some description
 */

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
