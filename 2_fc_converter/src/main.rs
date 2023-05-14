use std::io;

fn main() {
    println!("FC converter!");

    loop {
        println!("Enter 0 for CF or 1 for FC");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        let mode: u8 = mode.trim().parse().expect("Enter a number");
        let mut input = String::new();

        match mode {
            0 => {
                println!("Enter C");

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");

                let input: f64 = input.trim().parse().expect("Enter a number");

                println!("{}C is equal to {}F", input, celsius_to_fahrenheit(input))
            }
            1 => {
                println!("Enter F");

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");

                let input: f64 = input.trim().parse().expect("Enter a number");

                println!("{}F is equal to {}C", input, fahrenheit_to_celsius(input))
            }
            _ => println!("Invalid mode!"),
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
