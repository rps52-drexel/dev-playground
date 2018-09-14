use std::io;

fn main() {
    loop {
        println!("Enter f to convert to fahrenheit, c to convert to celsius or q to quit.");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        
        match choice.trim() {
            "f" => println!("{}", get_temp(choice)),
            "c" => println!("{}", get_temp(choice)),
            "q" => break,
            _ => println!("Bad choice!"),
        };
    }
}

fn get_temp(choice: String) -> f32 {
    loop {
        println!("Enter a temperature!");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        let choice: f32 = match choice.trim() {
            "f" => convert_fahrenheit(temp),
            "c" => convert_celsius(temp),
            _ => {
                println!("Bad choice");
                continue;
                }
            };
        return choice
        }
    }

    fn convert_fahrenheit(f: f32) -> f32 {
        let f = f - 32.0;
        f * 0.5556
    }

    fn convert_celsius(c: f32) -> f32 {
        let c = c * 1.8;
        c + 32.0
    }
