use std::fmt;

use exercise_utils::{ print_title, print_w_exercise_indent };

pub fn run() {
  println!("CH 2: FUNDAMENTALS");
  // pc1("EX 1 - Print Variables");
  pc2("EX 2 - Display Name");
  pc3("EX 3 - Personal Details");
  pc4("EX 4 - Star Pattern");
  // pc5("EX 5 - Sales Division Contribution");
  // pc6("EX 6 - Sqft to Acres");
  // pc7("EX 7 - Sales Tax");
  // pc8("EX 8 - Cookies Calories");
  // pc9("EX 9 - Miles per Gallon");
  // pc10("EX 10 - Test Average");
  pc12("EX 12 - String Manipulation");
  pc14("EX 14 - Male & Female Percentages");
  // pc15("EX 15 - Stock Commission");
  // pc16("EX 16 - Energy Drink Consumption");
  // pc17("EX 17 - Ingredient Adjuster");
  pc19("EX 19 - Stock Transaction");
}

// #################################
// ###        EXERCISES          ###
// #################################

fn pc1(title: &str) {
  print_title(title);

  let name: &str = "Neal Caffrey";
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

  struct FullName<'a> {
    first: &'a str,
    middle: &'a str,
    last: &'a str
  }

  impl fmt::Display for FullName<'_> {
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

  struct PersonalDetails<'a> {
    name: &'a str,
    address: &'a str,
    city: &'a str,
    state: &'a str,
    zip: &'a str,
    telephone_number: &'a str,
    college_major: &'a str
  }

  impl<'a> PersonalDetails<'a> {
    fn new(
      name: &'a str,
      address: &'a str,
      city: &'a str,
      state: &'a str,
      zip: &'a str,
      telephone_number: &'a str,
      college_major: &'a str
    ) -> Self {
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

  impl fmt::Display for PersonalDetails<'_> {
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

    pattern.push_str(format!("\nCapacity: {}, Size: {}\n", pattern.capacity(), pattern.len()).as_str());
    
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

  let drive_to_vt = CarTravelLog {
    distance_miles: 241.0,
    gallons_used: 7.32
  };

  print_w_exercise_indent(format!("Fuel efficiency={:.2} mpg", drive_to_vt.calc_fuel_efficiency()));

  struct CarTravelLog {
    distance_miles: f64,
    gallons_used: f64
  }

  impl CarTravelLog {
    fn calc_fuel_efficiency(&self) -> f64 {
      self.distance_miles as f64 / self.gallons_used as f64
    }
  }
}

fn pc10(title: &str) {
  print_title(title);

  let scores = [89.3, 74.7, 92.5];

  print_w_exercise_indent(format!("Avg score={}", calc_avg(scores)));

  fn calc_avg(arr: [f64; 3]) -> f64 {
    let mut sum = 0.0;
    for num in arr {
      sum += num;
    }
    return sum/arr.len() as f64;
  }
}

fn pc12(title: &str) {
  print_title(title);

  let city = "Tallinn";

  print_w_exercise_indent(format!("{} ({}) {}", city.to_ascii_uppercase(), city.len(), &city[..1]));
}

fn pc14(title: &str) {
  print_title(title);

  let class_headcount = 25;
  let male_count = 14;

  // ♂ ♀

  print_w_exercise_indent(format!("♂={:.0}%, ♀={:.0}%",
    male_count as f64 / class_headcount as f64 * 100.,
    (class_headcount as f64 - male_count as f64) / class_headcount as f64 * 100.)
  );
}


fn pc15(title: &str) {
  print_title(title);

  let trade = StockTransaction {
    shares_exchanged: 600,
    price_per_share: 21.77
  };

  print_w_exercise_indent(trade.print_receipt());

  struct StockTransaction {
    shares_exchanged: u32,
    price_per_share: f64
  }

  impl StockTransaction {
    const BROKER_COMMISSION: f64 = 0.02;

    fn calc_commission(&self) -> f64 {
      return self.shares_exchanged as f64 * self.price_per_share * Self::BROKER_COMMISSION;
    }

    fn print_receipt(&self) -> String {
      let mut receipt = String::with_capacity(200);

      receipt.push_str(&format!("{} shares at ${:.2}/share\n", self.shares_exchanged, self.price_per_share));
      receipt.push_str(&format!("\n{:14} ${:10.2}", "Transaction", self.shares_exchanged as f64 * self.price_per_share));
      receipt.push_str(&format!("\n{:14} ${:10.2}", "Commission", self.calc_commission()));
      receipt.push_str(&format!("\n{:->26}", ""));
      receipt.push_str(&format!("\n{:14} ${:10.2}", "Total", self.shares_exchanged as f64 * self.price_per_share + self.calc_commission()));

      receipt
    }
  }
}


fn pc16(title: &str) {
  print_title(title);

  print_w_exercise_indent(customer_preferences());

  fn customer_preferences() -> String {
    let customers_count = 12_467;
    let weekly_purchase_pct = 0.14;
    let prefer_citrus_pct = 0.64;

    return format!("{:20}{:5.0}\n{:20}{:5.0}\n{:20}{:5.0}",
      "Total Customers", customers_count,
      "Weekly Customers", customers_count as f64 * weekly_purchase_pct,
      "Prefer Citrus", customers_count as f64 * weekly_purchase_pct * prefer_citrus_pct
    );
  }
}

fn pc17(title: &str) {
  print_title(title);

  print_w_exercise_indent(cookie_ingredients(16));

  fn cookie_ingredients(num_cookies: u32) -> String {
    let sugar_cups = 1.5;
    let butter_cups = 1.0;
    let flour_cups = 2.75;
    let cookies_made = 48;

    let ratio = num_cookies as f64 / cookies_made as f64;

    let mut recipe = String::with_capacity(200);

    recipe.push_str(&format!("Makes {} cookies", num_cookies));
    recipe.push_str(&format!("\n{:8} {:4.2} cp", "Flour", flour_cups * ratio));
    recipe.push_str(&format!("\n{:8} {:4.2} cp", "Butter", butter_cups * ratio));
    recipe.push_str(&format!("\n{:8} {:4.2} cp", "Sugar", sugar_cups * ratio));

    recipe
  }
}

fn pc19(title: &str) {
  print_title(title);

  print_w_exercise_indent(calc_profit());

  fn calc_profit() -> String {
    let bought = StockTransaction::new("SP5", 1000, 32.87);
    let sold = StockTransaction::new("SP5", 1000, 33.92);

    print_w_exercise_indent(bought.print_receipt());
    print_w_exercise_indent(sold.print_receipt());

    let profit = sold.cost_of_shares - bought.cost_of_shares;
    let commissions_paid = bought.commission_paid + sold.commission_paid;

    let mut receipt = String::with_capacity(128);

    receipt.push_str(&format!("{:18} ${:10.2}", "Profit from Sale", profit));
    receipt.push_str(&format!("\n{:18} ${:10.2}", "Net Commission", -commissions_paid));
    receipt.push_str(&format!("\n{:->30}", ""));
    receipt.push_str(&format!("\n{:18} ${:10.2}", "Net Profit", profit - commissions_paid));

    receipt
  }
  

  struct StockTransaction<'a> {
    symbol: &'a str,
    shares_exchanged: u32,
    price_per_share: f64,
    cost_of_shares: f64,
    commission_paid: f64
  }

  impl<'a> StockTransaction<'a> {
    const BROKER_COMMISSION: f64 = 0.02;

    fn new(symbol: &'a str, shares_exchanged: u32, price_per_share: f64) -> Self {
      Self {
        symbol,
        shares_exchanged,
        price_per_share,
        cost_of_shares: shares_exchanged as f64 * price_per_share,
        commission_paid: shares_exchanged as f64 * price_per_share * Self::BROKER_COMMISSION
      }
    }

    fn print_receipt(&self) -> String {
      let mut receipt = String::with_capacity(200);

      receipt.push_str(&format!("({}) {} shares at ${:.2}/share\n", self.symbol, self.shares_exchanged, self.price_per_share));
      receipt.push_str(&format!("\n{:14} ${:10.2}", "Transaction", self.shares_exchanged as f64 * self.price_per_share));
      receipt.push_str(&format!("\n{:14} ${:10.2}", "Commission", self.commission_paid));
      receipt.push_str(&format!("\n{:->26}", ""));
      receipt.push_str(&format!("\n{:14} ${:10.2}\n", "Total", self.shares_exchanged as f64 * self.price_per_share + self.commission_paid));

      receipt
    }
  }
}