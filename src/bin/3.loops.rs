fn main () {
    let mut i = 10;

    loop {        
        println!("{}", i);

        i -= 1;

        if i == 0 {
            println!("Happy New Year !!");
            break;   
        }
    }
}