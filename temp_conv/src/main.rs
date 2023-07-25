use std::io;
use std::process; 


fn f_to_c(f_temp: f64) -> f64{
    (f_temp - 32.0) / (9.0 / 5.0)
}

fn c_to_f(c_temp: f64) -> f64{
    c_temp * (9.0 / 5.0) + 32.0
}

fn get_number_input() -> u64{
    loop{
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line\n");

        let number: u64 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!{"You did not enter a correct value, please try again.\n"};
                continue
            }
        };
        break number
    }
}

fn f_to_c_loop() {
    println!("Enter the temp you would like to convert.\n");
    let temp: f64 = get_number_input() as f64;
    let result: u64 = f_to_c(temp) as u64; 
    println!("{temp}C is equal to {result}F\n");
}

fn c_to_f_loop() {
    println!("Enter the temp you would like to convert.\n");
    let temp: f64 = get_number_input() as f64;
    let result: u64 = c_to_f(temp) as u64; 
    println!("{temp}C is equal to {result}F\n");
}

fn main() {
    println!("Welcome to C to F converter.\n");
    loop { 
        println!("1. C -> F | 2. F -> C| 3. Exit\n");
        let user_input: u64 = get_number_input();
        match user_input{
            1 => c_to_f_loop(),
            2 => f_to_c_loop(),
            3 => {
                println!("See you soon!\n");
                process::exit(0);
            },
            _ => println!("You did not enter a valid option u.u\n"),
        }
    }
}
