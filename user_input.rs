#![allow(dead_code)]

use std::io::{self, Write};

/* ----------------------------------
 * -------      EXECUTE       -------
 * ----------------------------------
 */

pub fn run() {
    basics();
    cast_input_as_u8();
}

// ##########################
// ###     USER INPUT     ###
// ##########################

fn basics() {
    print!("Enter a string: ");

    io::stdout().flush()                                        // flush() in std::io::Write
        .expect("Manual flush failed.");                        // Flush required to get entered input on same line as prompt text

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("The characters should be reprinted as is.");

    println!("You entered: {}", guess.trim_end())                     // Trim trailing newline
}

fn cast_input_as_u8() {
    let number: u8;

    loop {
        println!("Enter a 8-bit number below: ");

        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer)
            .expect("No characters should be able to crash the program.");
        
        number = match buffer.trim_end().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        break;                          // Only reached after successful parse
    }
    
    print!("You entered: {}", number);
}