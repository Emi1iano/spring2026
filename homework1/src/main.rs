fn assignment1() {
    const FREEZING_POINT_OF_WATER_FAHRENHEIT: f64 = 32.0;

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - FREEZING_POINT_OF_WATER_FAHRENHEIT) * (5.0/9.0)
    }
    fn celsius_to_fahrenheit(f: f64) -> f64 {
        f * 9.0 / 5.0 + FREEZING_POINT_OF_WATER_FAHRENHEIT
    }
    
    let mut temp: f64 = 100.0;
    println!("{}", fahrenheit_to_celsius(temp));
    for i in 1..=5 {
        println!("{}", fahrenheit_to_celsius(temp+(i as f64)));
    }
}
fn assignment2() {
    
}
fn assignment3() {
    
}

fn main() {
    assignment1();
    assignment2();
    assignment3();
}