
// this is an main function

fn main() {
    // let c = sum(5, 6);
    print!("{:?}", display_result(sum(5, 6)));
}

fn sum (a: i32, b: i32) -> i32 {
    a + b
}

fn display_result (result: i32) {
    println!("The sum is {}", result);
}
