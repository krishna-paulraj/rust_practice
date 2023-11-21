fn main () {
    let name = "krish";

    match name {
        "krish" => println!("Its my name"),
        "Karan" => println!("Its my brother name"),
        _ => println!("Its not my name"),           // _ is a catchall value
    }

    identify_true_or_false(true);                   // exercise 1
    identify_number(1);                             // exercise 2
}

// exercise 1

fn identify_true_or_false (i: bool) {
    match i {
        true => println!("its true"),
        false => println!("its false"),
    }
 }

// exercise 2

fn identify_number (a: i32) {
    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other number"),
    }
}