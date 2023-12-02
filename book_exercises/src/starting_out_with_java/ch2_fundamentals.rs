use std::fmt;

use crate::utils::{ print_title, print_w_exercise_indent };

pub fn run() {
  println!("CH 2: FUNDAMENTALS");
  pc1("EX 1 - Print Variables");
  pc2("EX 2 - Display Name");
  pc3("EX 3 - Personal Details");
  pc4("EX 4 - Star Pattern");
  pc5("EX 5 - Sales Division Contribution");
  pc6("EX 6 - Sqft to Acres");
  pc7("EX 7 - Sales Tax");
  pc8("EX 8 - Cookies Calories");
  // pc9("EX 9 - Miles per Gallon");
  // pc10("EX 10 - Test Average");
}

// #################################
// ###        EXERCISES          ###
// #################################

fn pc1(title: &str) {
  print_title(title);

  let name: &'static str = "Neal Caffrey";
  let age: u8 = 46;
  let monthly_salary: f64 = 1_600.0;

  print_w_exercise_indent(format!("Name: {}\nAge: {}\nSalary ${:.2}", name, age, monthly_salary));
}

fn pc2(title: &str) {
  print_title(title);

  let initials = Initials('n', 'g', 'c');

  let name = FullName {
    first: "Neal",
    middle: "George",
    last: "Caffrey"
  };

  print_w_exercise_indent(format!("Initials: {}\n{}", initials, name));

  struct Initials(char, char, char);

  impl fmt::Display for Initials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.", self.0.to_ascii_uppercase(), self.1.to_ascii_uppercase(), self.2.to_ascii_uppercase())
    }
  }

  struct FullName {
    first: &'static str,
    middle: &'static str,
    last: &'static str
  }

  impl fmt::Display for FullName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "First: {}\nMiddle: {}\nLast: {}", self.first, self.middle, self.last)
    }
  }
}

fn pc3(title: &str) {
  print_title(title);

  let sherlock = PersonalDetails::new(
    "William Sherlock Scott Holmes", 
    "221B Baker Street",
    "London",
    "Greater London",
    "NW1 6XE",
    "+44-20-7224-3688",
    "Chemistry"
  );

  print_w_exercise_indent(format!("{}", sherlock));

  struct PersonalDetails {
    name: &'static str,
    address: &'static str,
    city: &'static str,
    state: &'static str,
    zip: &'static str,
    telephone_number: &'static str,
    college_major: &'static str
  }

  impl PersonalDetails {
    fn new(name: &'static str, address: &'static str, city: &'static str, state: &'static str, zip: &'static str, telephone_number: &'static str, college_major: &'static str) -> Self {
      Self {
        name,
        address,
        city,
        state,
        zip,
        telephone_number,
        college_major
      }
    }
      
  }

  impl fmt::Display for PersonalDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}, {} {}\n{}\n{}",
          self.name,
          self.address,
          self.city, self.state, self.zip,
          self.telephone_number,
          self.college_major
        )
    }
  }
}

fn pc4(title: &str) {
  print_title(title);
  print_w_exercise_indent(format!("{}", create_star_pattern(4)));
  print_w_exercise_indent(format!("{}", create_star_pattern(7)));
  // print_w_exercise_indent(format!("{}", create_star_pattern(18)));

  fn create_star_pattern(rows: usize) -> String {
    let mut pattern = String::with_capacity(3 * rows.pow(2) + 32);

    pattern.push_str(format!("Rows={}\n\n", rows).as_str());

    for i in 0..rows {
      for j in 1..rows {
        if j < rows-i {
          pattern.push_str(" ");
        } else {
          pattern.push_str("* ");
        }
      }
      pattern.push_str("*\n");
    }
    
    for i in (1..=rows-1).rev() {
      for j in 0..rows-1 {
        if j < rows-i {
          pattern.push_str(" ");
        } else {
          pattern.push_str("* ");
        }
      }
      pattern.push_str("*\n");
    }

    // pattern.push_str(format!("\nCapacity: {}, Size: {}\n", pattern.capacity(), pattern.len()).as_str());
    
    pattern
  }
}

fn pc5(title: &str) {
  print_title(title);

  let company_revenue = 1_999_999.9975;
  let east_coast_contrib = 0.32;

  print_w_exercise_indent(
    format!("Total Revenue=${:.2}\nEast Coast Contibution=${:.2} ({}%)",
      company_revenue,
      sales_contribution(company_revenue, east_coast_contrib),
      east_coast_contrib * 100.0
    )
  );

  fn sales_contribution(total_revenue: f64, division_contribution: f64) -> f64 {
    return total_revenue * division_contribution;
  }
}

fn pc6(title: &str) {
  print_title(title);
  print_w_exercise_indent(format!("Acres={:.2}", sqft_to_acres(389_767.0)));

  fn sqft_to_acres(sqft: f64) -> f64 {
    sqft / 43_560.0
  }
}

fn pc7(title: &str) {
  print_title(title);

  print_w_exercise_indent(print_receipt(24.0, 0.04, 0.02));

  fn print_receipt(base_price: f64, state_tax: f64, county_tax: f64) -> String {
    let mut receipt = String::with_capacity(128);

    receipt.push_str(&format!("{:12} ${:>6.2}", "Price", base_price));
    receipt.push_str(&format!("\n{:12} ${:>6.2}", "State Tax", base_price*state_tax));
    receipt.push_str(&format!("\n{:12} ${:>6.2}", "County Tax", base_price*county_tax));
    receipt.push_str(&format!("\n{:->20}", ""));
    receipt.push_str(&format!("\n{:12} ${:>6.2}", "Total Tax", base_price * (state_tax+county_tax)));
    receipt.push_str(&format!("\n{:12} ${:>6.2}", "Total Price", base_price * (1.0+state_tax+county_tax)));

    // println!("\nCapacity: {}, Size: {}\n", receipt.capacity(), receipt.len());

    receipt
  }
}

fn pc8(title: &str) {
  print_title(title);

  let nutrition_label = BagCalorieInfo {
    cookies_per_bag: 40,
    servings_per_bag: 10,
    calories_per_bag: 3000
  };

  print_w_exercise_indent(calc_calories(12, nutrition_label));

  fn calc_calories(cookies_eaten: u16, calorie_info: BagCalorieInfo) -> String {
    return format!("Cookies eaten={},   Calories consumed={}", cookies_eaten, cookies_eaten * (calorie_info.calories_per_bag / calorie_info.cookies_per_bag));
  }

  struct BagCalorieInfo {
    cookies_per_bag: u16,
    servings_per_bag: u8,
    calories_per_bag: u16
  }
}

fn pc9(title: &str) {
  print_title(title);
}

fn pc10(title: &str) {
  print_title(title);
}