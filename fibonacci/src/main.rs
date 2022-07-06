fn main() {
    let result: u32 = fibonacci(43);

    println!("Result == {}", result);
}

fn fibonacci(count:u32) -> u32 {

    if count == 1 || count == 0{
        return count
    }

    fibonacci(count-1) + fibonacci(count-2)
}