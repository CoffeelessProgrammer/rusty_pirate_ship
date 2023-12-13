use exercise_utils::{ print_title, print_w_exercise_indent };

pub fn run() {
  println!("CH 3: Control Flow");
  // pc1("EX 1 - Roman Numerals");
  // pc2("EX 2 - Magic Dates");
  // pc3("EX 3 - Body Mass Index");
  // pc5("EX 5 - Weight");
  // pc7("EX 7 - Sorted Names");
}

#[allow(non_snake_case)]
fn pcXX(title: &str) {
  print_title(title);
  print_exercise_result("HelloWorld");

  fn print_exercise_result(input: &str) {
    print_w_exercise_indent(format!("{input} -> {}", exercise_logic(input)));
  }

  fn exercise_logic(input: &str) -> String {
    return "".to_string();
  }
}

// #################################
// ###        EXERCISES          ###
// #################################

fn pc1(title: &str) {
  print_title(title);

  print_as_roman_num(47);
  print_as_roman_num(2934);
  print_as_roman_num(838);
  print_as_roman_num(493);

  fn print_as_roman_num(num: u32) {
    print_w_exercise_indent(format!("{num} -> {}", convert_to_roman_numeral(num)));
  }

  fn convert_to_roman_numeral(mut num: u32) -> String {
    // I, V, X, L, C, D, M
    if num == 0 { return "nulla".to_string() }

    let mut roman = String::with_capacity(16);
    let lookup = [
      (1000, "M"),
      (900, "CM"),
      (500, "D"),
      (400, "CD"),
      (100, "C"),
      (90, "XC"),
      (50, "L"),
      (40, "XL"),
      (10, "X"),
      (9, "IX"),
      (5, "V"),
      (4, "IV"),
      (1, "I")
    ];

    let mut multiple;

    for (key, val) in lookup {
      multiple = num / key;
      num %= key;
      roman.push_str(&val.repeat(multiple as usize));
      if num == 0 { break }
    }

    return roman;
  }
}

fn pc2(title: &str) {
  print_title(title);
  print_is_magic_date(4, 11, 2044);
  print_is_magic_date(5, 13, 3060);
  print_is_magic_date(11, 12, 5132);
  print_is_magic_date(12, 30, 8358);
  print_is_magic_date(12, 28, 8336);

  fn print_is_magic_date(month: u32, day: u32, year: u32) {
    print_w_exercise_indent(format!("{month}/{day}/{} -> {}", year%100, is_magic_date(month, day, year)));
  }

  fn is_magic_date(month: u32, day: u32, year: u32) -> bool {
    year%1000 == month * day
  }
}

fn pc3(title: &str) {
  print_title(title);
  print_bmi(70, 150.0);
  print_bmi(551, 10_250.0);

  fn print_bmi(height_inches: u16, weight_lbs: f32) {
    let result = calc_bmi(height_inches, weight_lbs);
    print_w_exercise_indent(format!("{:} in, {:.0} lbs -> {:.1} ({})", height_inches, weight_lbs, result.0, result.1));
  }

  fn calc_bmi(height_inches: u16, weight_lbs: f32) -> (f32, String) {
    let bmi = weight_lbs * 703.0 / (height_inches as f32).powi(2);
    ( bmi, if bmi < 18.5 { "underweight".to_owned() }
      else if bmi > 25.0 { "overweight".to_owned() }
      else { "healthy".to_owned() } )
  }
}

fn pc5(title: &str) {
  print_title(title);
  print_planetary_weight(68.0, Planet::JUPITER);
  print_planetary_weight(4649.3218, Planet::MERCURY);

  fn print_planetary_weight(mass_kg: f32, planet: Planet) {
    print_w_exercise_indent(format!("{}", calc_planetary_weight_kg(mass_kg, &planet)));
  }

  fn calc_planetary_weight_kg(mass_kg: f32, planet: &Planet) -> String {
    let weight_lbs = mass_kg * planet.1 * 0.224809;
    let earth_weight_lbs = mass_kg * Planet::EARTH.1 * 0.224809;
    return format!("{} -> {:.1} lbs. (🌎 {:.0} lbs.)", planet.0, weight_lbs, earth_weight_lbs);
  }

  struct Planet<'a>(&'a str, f32);
  // Gravity in m/s^2: Mercury=3.7, Earth=9.807, Jupiter=24.79, Neptune=11.15

  impl<'a> Planet<'a> {
    const EARTH: Planet<'a> = Planet("Earth", 9.807);
    pub const MERCURY: Planet<'a> = Planet("Mercury", 3.7);
    pub const JUPITER: Planet<'a> = Planet("Jupiter", 25.79);
    pub const NEPTUNE: Planet<'a> = Planet("Neptune", 11.15);
  }
}


fn pc7(title: &str) {
  print_title(title);
  sort_names(vec!["Midoriya", "shoto", "Ochaco", "bakugo"]);

  fn sort_names(mut names: Vec<&str>) {
    // names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    names.sort_by_key(|name| name.to_lowercase());
    print_w_exercise_indent(format!("{:?}", names));
  }
}