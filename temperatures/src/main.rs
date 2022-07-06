fn main() {


    println!("Celsius to farenheit : {} to {}", 10.0, c_to_f(10.0));
    println!("Farenheit to Carenheit : {} to {}", 10.0, f_to_c(10.0));

}

fn c_to_f(celsius: f32) -> f32 {
    (celsius * 9.0/5.0) + 32.0
}

fn f_to_c(farheneit: f32) -> f32 {
    (farheneit - 32.0) * 5.0/9.0
}
