use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    println!("Please input the value");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let number_input = get_number(user_input);
    let result = fibonacci_sequence(number_input);

    let message = String::from(format!("Result: {result}"));
    print_message(message)
}

fn print_message(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn fibonacci_sequence(sequence_number: i32) -> i32 {
    if sequence_number == 0 {
        return 0;
    }

    if sequence_number == 1 {
        return 1;
    }

    return fibonacci_sequence(sequence_number - 1) + fibonacci_sequence(sequence_number - 2);
}

fn get_number(user_input: String) -> i32 {
    user_input.trim().parse().expect("Please type a number!")
}
