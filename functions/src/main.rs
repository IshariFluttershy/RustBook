fn main() {
    println!("Hello, world!");

    let x = plus_one(five());

    another_function(x, 'h');


}

fn another_function(x: i32, y: char) {
    println!("The value of X and Y are: {} and {}", x, y);

}
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}