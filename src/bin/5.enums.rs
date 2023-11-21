enum Directions {
    Up,
    Down,
    Left,
    Right
}

enum Colours {
    Red,
    Green,
    Blue,
    Black
}

fn main () {
    which_way(Directions::Up);
    
    print_color(Colours::Red);                  // exercise 1
}

fn which_way (go: Directions) {
    match go {
        Directions::Up => println!("Go up!"),
        Directions::Down => println!("Go down!"),
        Directions::Left => println!("Go left!"),
        Directions::Right => println!("Go right!")
    }
}

// exercise 1 : Create an enum of colours and print out the colour values;

fn print_color (color: Colours) {
    match color {
        Colours::Red => println!("Its a Red"),
        Colours::Green => println!("Its a Green"),
        Colours::Blue => println!("Its a Blue"),
        Colours::Black => println!("Its a Black")
    }
}