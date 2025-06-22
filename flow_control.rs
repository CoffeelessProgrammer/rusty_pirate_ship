/* ----------------------------------
 * -------      EXECUTE       -------
 * ----------------------------------
 */

pub fn run() {
    match_arms();
    conditionals();
    loops();
}

// ############################
// ###     FLOW CONTROL     ###
// ############################

fn match_arms() {                       // (≡java switch-case)
    for i in [-275, -273, -50, 0, 5, 12, 20, 28, 38] {
        temp_description(i);
    }

    fn temp_description(t: i16) {
        match t {
            i16::MIN..-273 => println!("Sub -273.15°C temp invalid"),
            -273..=0 => println!("Freezing"),
            1..12 => println!("Chilly"),
            12..20 => println!("Temperate"),
            20..=28 => println!("Warm"),
            // _ => println!("Hot, unsafe temperatures"),              // _ = default/catch-all
            x => println!("Hot, unsafe temperature: {x}°C")       // x = named default/catch-all
        }
    }
}

fn conditionals() {
    let battery: u8 = 42;
    
    if battery > 42 {
        // Battery high
    } else if battery > 16 {
        // Battery low
    } else {
        // Battery critical
    }

    let count: u8 = 16;
    let label: &'static str = if count == 1 { "donut" } else { "donuts" };      // if-else blocks return val in Rust
    println!("Inventory: {} {}", count, label);
}

fn loops() {
    let numbers = [42, 2, 20, 4, 5];

    for num in numbers {
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

    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 0],
    ];

    'outer: for row in 0..grid.len() {              // Loop labeling
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 {
                println!("Found a 1 at ({}, {})", row, col);
                break 'outer;
            }
        }
    }

    // 'loop' can return a value w/ break
    // See user_input.rs for infinite 'loop' example
}