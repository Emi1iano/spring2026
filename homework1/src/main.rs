fn assignment1() {
    const FREEZING_POINT_OF_WATER_FAHRENHEIT: f64 = 32.0;

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - FREEZING_POINT_OF_WATER_FAHRENHEIT) * (5.0/9.0)
    }
    fn celsius_to_fahrenheit(f: f64) -> f64 {
        f * 9.0 / 5.0 + FREEZING_POINT_OF_WATER_FAHRENHEIT
    }
    println!("Assignment 1-----------------");
    let mut temp: f64 = 100.0;
    println!("{}", fahrenheit_to_celsius(temp));
    for i in 1..=5 {
        println!("{}", fahrenheit_to_celsius(temp+(i as f64)));
    }
    println!("Assignment 1-----------------");
}
fn assignment2() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    println!("Assignment 2-----------------");
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for num in numbers {
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
        if num % 3 == 0 {
            if num % 5 == 0 {
                println!("FizzBuzz");
                continue;
            }
            println!("Fizz");
            continue;
        }
        if num % 5 == 0 {
            println!("Buzz");
            continue;
        }
    }
    
    let mut counter = 0;
    let mut total = 0;
    while counter != 10 {
        total += numbers[counter];
        counter += 1;
    }
    println!("{}", total);

    let mut counter = 0;
    let mut aux = numbers[0];
    let biggest = loop {
        if counter >= numbers.len() {
            break aux;
        }
        if aux < numbers[counter] {
            aux = numbers[counter];
        }
        counter += 1;
    };
    println!("{}", biggest);
    println!("Assignment 2-----------------");
}
fn assignment3() {
    let mut secret = 13;
    
    fn check_guess(guess: i32, secret: i32) -> i32 {
        if secret == guess {
            return 0;
        }
        if secret > guess {
            return -1;
        }
        return 1;
        
    }

    println!("Assignment 3-----------------");
    let mut count = 0;
    let mut guess = 1;
    loop {
        let result = check_guess(guess, secret);
        if result == 0 {
            break;
        }
        else if result == 1 {
            println!("Too high");
            guess -= 10;
        }
        else if result == -1 {
            println!("Too low");
            guess += 9;
        }
        count += 1;
    }
    println!("Took you {} attempts to guess", count);
    println!("Assignment 3-----------------");
}

fn main() {
    assignment1();
    assignment2();
    assignment3();
}