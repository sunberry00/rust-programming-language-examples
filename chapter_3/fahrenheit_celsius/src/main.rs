use std::io;
fn main() {
    println!("Please input temperature in Celsius");

    let mut temp_c = String::new();

    io::stdin()
        .read_line(&mut temp_c)
        .expect("Failed to read a line");

    let temp_c: i32 = temp_c.trim().parse().expect("Failed to parse string");

    let temp_f: i32 = (temp_c * 9 / 5) + 32;

    println!("Temperature in Fahrenheit is {temp_f}");
}
