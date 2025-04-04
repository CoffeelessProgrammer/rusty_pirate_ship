#![allow(dead_code)]

use std::fmt;
// 'use' is 100% optional, fully qual. name can be used each time instead

/* ----------------------------------
 * -------      EXECUTE       -------
 * ----------------------------------
 */

pub fn run() {
    // basics();
    // debug_trait();
    // display_trait();
    // display_list();
    loops();
}

// ############################
// ###    SYNTAX & PRINT    ###
// ############################

fn basics() {
    let country = "Japan";
    println!("Hello {}!", country);
    println!("Hello {0}!", "Switzerland");                      // Positional args
    println!("Hello {country}!", country="Iceland");            // Named args

    println!("0b{number:>8b}", number=42);                          // Right-justify(>), width=8
    println!("0b{number:<8b}", number=11);                          // Left-justify (<) by pointing arrow left
    println!("--{number:^12b}--", number=11);                       // Center-align (^), if uneven, more padding on right
    println!("0b{number:0>8b}", number=42);                         // Syntax: {arg:fill direction width fmt_char}
    println!("0x{number:f>width$x}", number=75, width=4);           // Named args in format specifier by appending `$`.
}

fn debug_trait() {

    #[derive(Debug)]                                    // Automatic `fmt::Debug` impl.
    struct Structure(i32, f32);
    
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{1} {0:?} is the {agent:?} name.", "Peter", "Burke", agent="agents's");
    println!("{:?}", Deep(Structure(42, 3.141592)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter Burke";
    let age = 42;
    let agent = Person { name, age };

    println!("{:#?}", agent);                           // ':#?' pretty print, expanded version
}

fn display_trait() {

    #[derive(Debug)]
    struct Vector2D(i64, i64);

    impl fmt::Display for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {           
            write!(f, "({}, {})", self.0, self.1)                   // Use `self.number` to refer to each positional data point
        }
    }

    impl fmt::Binary for Vector2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let magnitude = (self.0 * self.0 + self.1 * self.1) as f64;
            let magnitude = magnitude.sqrt();

            let decimals = f.precision().unwrap_or(3);
            let string = format!("{magnitude:.decimals$}");
            f.pad_integral(true, "", &string)
        }
    }

    let range = Vector2D(-128, 127);
    println!("Range: {range}\n");                         // Implicitly displayed b/c fmt::Display impl.

    let vector = Vector2D(0, 14);

    println!("Binary: {:10.2b}", vector);
    println!("Debug: {:?}\n", vector);

    
    #[derive(Debug)]
    struct Point2D{x: f64, y: f64}

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {            
            write!(f, "x: {}, y: {}", self.x, self.y)               // Referencing named fields
        }
    }

    let point = Point2D{x: 3.3, y: 7.2};

    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

fn display_list() {
    struct Pokemon { name: String, level: u8 }

    impl fmt::Display for Pokemon {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Pokemon {{ name: {}, level: {} }}", self.name, self.level)
        }
    }

    struct List(Vec<Pokemon>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
    
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "\n    {count}. {}", v)?;
            }

            write!(f, "\n]")
        }
    }

    let v = List(vec![
        Pokemon { name: String::from("Chikorita"), level: 15}, 
        Pokemon { name: String::from("Totodile"), level: 11},
        Pokemon { name: String::from("Cyndaquil"), level: 12}
    ]);
    println!("{}", v);
}


// #########################
// ###      STRINGS      ###
// #########################


// ############################
// ###      CONTAINERS      ###
// ############################


// ############################
// ###     FLOW CONTROL     ###
// ############################

fn loops() {
    let numbers = [42, 2, 20, 4, 5];

    for num in numbers.iter() {
        print!("{} ", num)
    }

    let sentence = "The quick brown fox.";
    let mut itr = sentence.split(' ').peekable();

    while let Some(token) = itr.next() {
        print!("{} ", token);
        if itr.peek().is_none() {
            println!();
        }
    }

    // See user_input.rs for infinite 'loop' example, 'break' to exit
}
