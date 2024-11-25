use std::io;

fn main() {
    //create a mutable string to hold the input
    let mut input_temperature = String::new();
    let mut operation = String::new();
    loop {
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit program");

        //Read the input 
        println!("Please enter your choice: ");
        io::stdin().read_line(&mut operation).unwrap();

        let operation: i32 = operation.trim().parse().expect("Please enter a number :");
        if operation == 1 {
            println!("1. Convert Fahrenheit to Celsius...");
            println!("Please enter Temperature in F: ");
            io::stdin().read_line(&mut input_temperature).unwrap();
            let temperature: f32 = input_temperature.trim().parse().expect("Please enter a valid floating point temperature");
            let celsius = (temperature - 32.0) * 5.0 / 9.0;
            println!("Temperature in Celsius: {}", celsius);
        }else if operation == 2{
            println!("2. Convert Celsius to Fahrenheit...");
            println!("Please enter Temperature in C: ");
            io::stdin().read_line(&mut input_temperature).unwrap();
            let temperature: f32 = input_temperature.trim().parse().expect("Please enter a valid floating point temperature");
            let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
            println!("Temperature in Fahrenheit: {}", fahrenheit);
        }else{
            println!("3. Exit program");
            break;
        }

    }


}