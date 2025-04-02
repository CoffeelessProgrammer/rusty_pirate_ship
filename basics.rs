#![allow(dead_code)]

use std::fmt;                       // 'use' is 100% optional, fully qual. name can be used each time instead

/* ----------------------------------
 * -------      EXECUTE       -------
 * ----------------------------------
 */

pub fn run() {
    // basics();
    // debug_trait();
    // display_trait();
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
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {           
            write!(f, "({}, {})", self.0, self.1)                   // Use `self.number` to refer to each positional data point
        }
    }

    let range =   MinMax(-128, 127);
    println!("Range: {range}\n");                         // Implicitly displayed b/c fmt::Display impl.
    
    #[derive(Debug)]
    struct Point2D{x: f64, y: f64}

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {            
            write!(f, "x: {}, y: {}", self.x, self.y)               // Referencing named fields
        }
    }

    let minmax = MinMax(0, 14);

    println!("Display: {}", minmax);
    println!("Debug: {:?}\n", minmax);

    let point = Point2D{x: 3.3, y: 7.2};

    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

// ############################
// ###      DATA TYPES      ###
// ############################


// ############################
// ###      CONTAINERS      ###
// ############################


// ##########################
// ###      TEMPLATE      ###
// ##########################