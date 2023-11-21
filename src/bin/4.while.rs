fn main () {
    let mut a = 0;

    while a <= 10 {
        println!("{}", a);
        a += 1;
    }

    count_down();   // exercise 1
}

// exercise 1: Counts from 5 to 1 and then prints "Happy New Year"

fn count_down () {
    let mut b = 5;

    while b > 0 {
        println!("{}", b);
        if b == 1 {
            println!("Happy New Year !!");
        }
        b -= 1;
    }
}

