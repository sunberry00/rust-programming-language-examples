use std::io;

fn main() {
    println!("Please enter a number for generating a Fibonacci number:");
    let mut n: String = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read a line");

    let n: u32 = n.trim().parse().expect("Failed to parse a string");
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    let result: u32 = if n == 1 {
        first
    } else if n == 2 {
        second
    } else {
        let mut counter: u32 = 3;
        while counter - 1 != n {
            let temp = second;
            second = first + second;
            first = temp;
            counter += 1;
        }
        second
    };

    println!("{n}th Fibonacci number is {result} ");
}
