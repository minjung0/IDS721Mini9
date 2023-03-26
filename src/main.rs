use std::io;

// integer to roman
fn main() {
    println!("Integer to Roman");

    println!("Please input an integer: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please type a number!");

    println!("Roman: {}", int_to_roman(input));
}

fn int_to_roman(mut num: i32) -> String {
    let mut result = String::new();
    let mut aux: i32;
    let rom = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    let mut i = 0;

    while num > 0 {
        aux = num % 10;
        num = num / 10;
        match aux {
            0..=3 => {
                for _ in 0..aux { result.insert(0, rom[i]); }
            },
            4 => {
                result.insert(0, rom[i]);
                result.insert(1, rom[i + 1]);
            },
            5..=8 => {
                result.insert(0, rom[i + 1]);
                for _ in 0..(aux-5) { result.insert(1, rom[i]); }
            },
            9 => {
                result.insert(0, rom[i]);
                result.insert(1, rom[i + 2]);
            }
            _ => {}
        }
        i += 2;
    }
    return result;
}
