#![allow(dead_code, unused_variables)]

/* ----------------------------------
 * -------      EXECUTE       -------
 * ----------------------------------
 */

pub fn run() {
    enums();
    structs();
}

// ######################
// ###      ENUM      ###
// ######################

fn enums() {
    let piggy_bank = [Coin::Nickel, Coin::Penny, Coin::Quarter, Coin::Dime, Coin::Penny];
    let mut savings = 0;
    for coin in piggy_bank {
        savings += value_in_cents(coin);
    }

    println!("Balance: {savings}¢");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }

    // ----------------------------------------------------

    enum Operation {
        Add(u8, u8),
        Mul(u8, u8),
        Sub { first: u8, second: u8 },
        Div { dividend: u8, divisor: u8 },
    }

    impl Operation {
        fn result(&self) -> u8 {
            match self {
                Self::Add(a, b) => a + b,                           // Self can replace enum name, e.g. Operation
                Self::Sub { first, second } => first - second,
                Self::Mul(a, b) => a * b,
                Self::Div { dividend, divisor } => dividend / divisor,
            }
        }
    }

    let op = Operation::Sub {
        first: 75,
        second: 20
    };
    println!("Result: {}", op.result());

}

// ########################
// ###      STRUCT      ###
// ########################

fn structs() {
    struct PkmnRegion(String, u8);        // Tuple struct
    let hoenn = PkmnRegion("Hoenn".to_string(), 3);

    #[derive(Debug)]
    enum PkmnType {
        Water, Fire, Grass
    }

    struct Pokemon {                    // struct
        national_dex_no: u16,
        name: String,
        level: u8,
        weight_kg: f32,
        battle_type: PkmnType
    }

    let mut bulbasaur = Pokemon {
        national_dex_no: 0001,
        name: String::from("Bulbasaur"),
        level: 15,
        weight_kg: 6.9,
        battle_type: PkmnType::Grass
    };

    println!("{} - lvl. {}", bulbasaur.name, bulbasaur.level);

    impl Pokemon {
        fn new(name: String, dex_no: u16, weight_kg: f32, battle_type: PkmnType) -> Pokemon {
            Pokemon {
                national_dex_no: dex_no,
                name, level: 1,
                weight_kg: weight_kg,
                battle_type: battle_type
            }
        }   // Associated function (≡java static) called via double-colon, e.g. Pokemon::new()

        fn get_name(&self) -> &str {
            &self.name
        }
        fn level_up(&mut self) {
            self.level += 1;
        }
        fn evolve(self, evolution: &str) {
            println!("{} is evolving into {}!", self.name, evolution);
        }
    }

    bulbasaur.level_up();
    println!("{} - lvl. {}", bulbasaur.get_name(), bulbasaur.level);
    bulbasaur.evolve("Ivysaur");

    let ivysaur =  Pokemon::new("Ivysaur".to_string(), 2, 13.0, PkmnType::Grass);
    println!("{} is a {:?} type.", ivysaur.name, ivysaur.battle_type);
}


// #########################
// ###      STRINGS      ###
// #########################


// ############################
// ###      CONTAINERS      ###
// ############################


// ############################
// ###      DATA TYPES      ###
// ############################