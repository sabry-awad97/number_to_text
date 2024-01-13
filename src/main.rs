use std::io;

fn number_to_text(number: i64) -> String {
    if number == 0 {
        return "Zero".to_string();
    }

    let mut words = Vec::new();

    if number < 0 {
        words.push("Minus".to_string());
    }

    words.extend(convert(number.abs()));

    words.join(" ")
}

fn convert(number: i64) -> Vec<String> {
    let mut words = Vec::new();

    if number >= 1_000_000_000_000_000_000 {
        words.extend(convert_large_number(
            number,
            1_000_000_000_000_000_000,
            "Sextillion",
        ));
    } else if number >= 1_000_000_000_000_000 {
        words.extend(convert_large_number(
            number,
            1_000_000_000_000_000,
            "Quintillion",
        ));
    } else if number >= 1_000_000_000_000 {
        words.extend(convert_large_number(
            number,
            1_000_000_000_000,
            "Quadrillion",
        ));
    } else if number >= 1_000_000_000 {
        words.extend(convert_large_number(number, 1_000_000_000, "Trillion"));
    } else if number >= 1_000_000 {
        words.extend(convert_large_number(number, 1_000_000, "Billion"));
    } else if number >= 1_000 {
        words.extend(convert_large_number(number, 1_000, "Million"));
    } else {
        words.extend(convert_small_number(number));
    }

    words
}

fn convert_large_number(number: i64, divisor: i64, unit: &str) -> Vec<String> {
    let quotient = number / divisor;
    let remainder = number % divisor;

    let mut words = Vec::new();
    if quotient != 0 {
        words.extend(convert_small_number(quotient));
        words.push(unit.to_string());
    }

    if remainder != 0 {
        if quotient != 0 {
            words.push("and".to_string());
        }
        words.extend(convert(remainder));
    }

    words
}

fn convert_small_number(number: i64) -> Vec<String> {
    let mut words = Vec::new();

    if number >= 100 {
        words.push(format!("{} Hundred", ordinalize((number / 100) as usize)));
    }

    let remainder = number % 100;

    if remainder > 0 {
        if !words.is_empty() {
            words.push("and".to_string());
        }

        if remainder < 20 {
            words.push(ordinalize(remainder as usize));
        } else {
            words.push(convert_tens(remainder / 10));
            if remainder % 10 > 0 {
                words.push(ordinalize((remainder % 10) as usize));
            }
        }
    }

    words
}

fn convert_tens(number: i64) -> String {
    match number {
        2 => "Twenty".to_string(),
        3 => "Thirty".to_string(),
        4 => "Forty".to_string(),
        5 => "Fifty".to_string(),
        6 => "Sixty".to_string(),
        7 => "Seventy".to_string(),
        8 => "Eighty".to_string(),
        9 => "Ninety".to_string(),
        _ => String::new(),
    }
}

fn ordinalize(num: usize) -> String {
    match num {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        11 => "Eleven".to_string(),
        12 => "Twelve".to_string(),
        13 => "Thirteen".to_string(),
        14 => "Fourteen".to_string(),
        15 => "Fifteen".to_string(),
        16 => "Sixteen".to_string(),
        17 => "Seventeen".to_string(),
        18 => "Eighteen".to_string(),
        19 => "Nineteen".to_string(),
        _ => unreachable!(),
    }
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i64>() {
        Ok(value) => println!("Textual representation: {}", number_to_text(value)),
        Err(_) => println!("Invalid input. Please enter a valid number."),
    }
}
