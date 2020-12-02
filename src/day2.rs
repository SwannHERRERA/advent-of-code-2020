use std::{fs::File, io::{BufRead, BufReader}};

pub fn day2() {
  parse_file("./file/day2.txt");
}

fn parse_file(path: &str) {
  let file = File::open(path).unwrap();
  let mut count_password_valid: usize = 0;
  let reader = BufReader::new(file);
  for line in reader.lines() {
    let str_line = line.unwrap();
    let vev: Vec<&str> = str_line.split(' ').collect();
    let password = create_password(vev);
    if check_password(password) {
      count_password_valid += 1;
    }
  }
  println!("{}", count_password_valid);
}

#[derive(Clone)]
struct Password {
  range: [usize; 2],
  letter: char,
  password: String,
}

fn create_password(line: Vec<&str>) -> Password {
  if line.len() != 3 {
    panic!("error line corrupted");
  }
  let range: Vec<usize> = line[0].split("-").map(|s| s.parse().unwrap()).collect();
  let letter = line[1].chars().nth(0).unwrap();
  let password = line[2].to_string();
  return Password {
    range: [range[0], range[1]],
    letter,
    password,
  }
}


fn check_password(password: Password) -> bool {
  if check_first_range_contain_letter(password.clone()) && !check_2gd_range_contain_letter(password.clone()) {
    return true
  }
  if !check_first_range_contain_letter(password.clone()) && check_2gd_range_contain_letter(password.clone()) {
    return true;
  }
  return false;
}

fn check_2gd_range_contain_letter(password: Password) -> bool {
  if password.password.chars().nth(password.range[1]-1).unwrap() == password.letter {
    true
  } else {
    false
  }
}

fn check_first_range_contain_letter(password: Password) -> bool {
  if password.password.chars().nth(password.range[0]-1).unwrap() == password.letter {
    true
  } else {
    false
  }
}