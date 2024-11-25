fn main() {
    loop {
        println!("looping...");
        break;
    }

    let mut num = 3;

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }
    println!("Increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.");
    println!("The safety and conciseness of for loops make them the most commonly used loop construct in Rust.");

    for num in (0..4).rev() {
        println!("{}", num);
    }

    //Returning Values from Loops
    let mut i: usize = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 10;
        }
    };
    println!("{}", result);

    //Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
