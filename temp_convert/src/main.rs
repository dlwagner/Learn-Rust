//Convert temperature between Fahrenheit and Celcius
use std::io;

fn main() {
    println!("Choose option to convert between Fahrenheit and Celsius:");
    println!("1. Convert Fahrenheit to Celsius");
    println!("2. Convert Celsius to Fahrenheit");
    println!("3. Exit");

    loop {

        println!("");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        if choice == 1 {
            println!("Enter Fahrenheit degrees:");
            let mut f = String::new();

            io::stdin().read_line(&mut f)
            .expect("Failed to read line");

            let f: f32 = match f.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let f = f_to_c(f);
            println!("Result: {}", f);
            println!("Convert again: ");

        } else if choice == 2 {
            println!("Enter Celsius degrees:");
            let mut c = String::new();
                                                                                                                                    io::stdin().read_line(&mut c)                                                                                           .expect("Failed to read line");
                                                                                                                                    let c: f32 = match c.trim().parse() {
                Ok(num) => num,                                                                                                         Err(_) => continue,
            };
                                                                                                                                    let c = c_to_f(c);                                                                                                      println!("Result: {}", c);                                                                                              println!("Convert again: ");  

        } else if choice == 3 {
            println!("Goodbye");
            break;
        } else {
            println!("Invalid choice, please choose again");
        }

    }

    fn f_to_c(x: f32) -> f32 {
        (x - 32.0)*0.5556
    }

    fn c_to_f(x: f32) -> f32 {
        (x*1.8) + 32.0
    }
}

