fn main () {
    let a = 5;
    
    if a > 6 {
        println!("greater than 6")
    } else {
        println!("less than 6")
    }

    true_or_false(false);   // exercise 2
    checking_integer(5);    // exercise 3
 }

// exercise 2

fn true_or_false (a: bool) {
    if a == true {
        println!("Hello");
    } else {
        println!("GoodBye");
    }
}

// exercise 3

fn checking_integer (i: i32) {
    if i > 5 {
        println!("> 5")
    } else if i < 5{
        println!("< 5")
    } else {
        println!("= 5")
    }
}
