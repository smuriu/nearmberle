use ran::{ran_urange, set_seeds};
use near_sdk::env;

fn ran_u8range(min: u8, max: u8) -> u8 {
  ran_urange(min.into(), max.into()).try_into().unwrap()
}

pub trait EquationGenerator {
  fn gen_equation(&self) -> String;
}

struct Puzzle1;
impl EquationGenerator for Puzzle1 {
  fn gen_equation(&self) -> String {
    let a: u8 = ran_u8range(91, 99);
    let b_floor = 100 - a;
    let b: u8 = if b_floor < 9 {
      ran_u8range(b_floor, 9)
    } else {
      9
    };
    let c = a + b;

    match ran_u8range(1, 3) {
      1 => format!("{}+{}={}", a, b, c), // a + b = c
      2 => format!("{}-{}={}", c, b, a), // c - b = a
      _ => format!("{}-{}={}", c, a, b), // c - a = b
    }
  }
}

struct Puzzle2;
impl EquationGenerator for Puzzle2 {
  fn gen_equation(&self) -> String {
    let a = 99u16;
    let b: u16 = ran_u8range(2, 9).into();
    let c = a * b;

    match ran_u8range(1, 3) {
      1 => format!("{}*{}={}", a, b, c), // a * b = c
      2 => format!("{}/{}={}", c, a, b), // c / a = b
      _ => format!("{}/{}={}", c, b, a), // c / b = a
    }
  }
}

struct Puzzle3;
impl EquationGenerator for Puzzle3 {
  fn gen_equation(&self) -> String {
    let a: u8 = 12;
    let b: u8 = 9;
    let c: u8 = 108;

    match ran_u8range(1, 3) {
      1 => format!("{}*{}={}", a, b, c), // a * b = c
      2 => format!("{}/{}={}", c, a, b), // c / a = b
      _ => format!("{}/{}={}", c, b, a), // c / b = a
    }
  }
}

struct Puzzle4;
impl EquationGenerator for Puzzle4 {
  fn gen_equation(&self) -> String {
    let a: u8 = ran_u8range(5, 9);
    let b: u8 = ran_u8range(2, 9);
    let c: u8 = a * b;
    let d: u8 = ran_u8range(1, 9);
    let e: u8 = c + d;

    match ran_u8range(0, 1) {
      0 => format!("{}*{}+{}={}", a, b, d, e), // a * b + d = e
      _ => format!("{}*{}+{}={}", b, a, d, e), // b * a + d = e
                                                   // _ => format!("{}-{}={}", e, d, c),       // e - d = c
    }
  }
}

struct Puzzle5;
impl EquationGenerator for Puzzle5 {
  fn gen_equation(&self) -> String {
    let a  = 9u8;
    let b = a;
    let c  = 3u8;
    let d  = 27u8;

    match ran_u8range(1, 3) {
      1 => format!("{}*{}/{}={}", a, b, c, d), // a * b / c = d
      2 => format!("{}*{}/{}={}", a, b, d, c), // a * b / d = c
      _ => format!("{}/{}*{}={}", d, a, c, b), // d / a * c = b
    }
  }
}

struct Puzzle6;
impl EquationGenerator for Puzzle6 {
  fn gen_equation(&self) -> String {
    let a = ran_u8range(10, 89);
    let b = ran_u8range(10, 99 - a);
    let c = a + b;

    match ran_u8range(1, 4) {
      1 => format!("{}+{}={}", a, b, c), // a + b = c
      2 => format!("{}+{}={}", b, a, c), // b + a = c
      3 => format!("{}-{}={}", c, a, b), // c - a = b
      _ => format!("{}-{}={}", c, b, a), // c - b = a
    }
  }
}

struct Puzzle7;
impl EquationGenerator for Puzzle7 {
  fn gen_equation(&self) -> String {
    let a = ran_u8range(1, 9);
    let b_floor = 11 - a;
    let b: u8 = if b_floor < 9 {
      ran_u8range(b_floor, 9)
    } else {
      9
    };
    let c = ran_u8range(1, 9);
    let d = a + b + c;

    match ran_u8range(1, 8) {
      1 => format!("{}+{}+{}={}", a, b, c, d),
      2 => format!("{}+{}+{}={}", b, a, c, d),
      3 => format!("{}+{}+{}={}", c, a, b, d),
      4 => format!("{}+{}+{}={}", c, b, a, d),
      5 => format!("{}-{}-{}={}", d, a, b, c),
      6 => format!("{}-{}-{}={}", d, b, c, a),
      7 => format!("{}-{}-{}={}", d, a, c, b),
      _ => format!("{}-{}-{}={}", d, b, a, c),
    }
  }
}

pub fn equation_generator() -> Box<dyn EquationGenerator> {
  set_seeds(env::block_timestamp());

  match ran_urange(1, 7) {
    1 => Box::new(Puzzle1 {}),
    2 => Box::new(Puzzle2 {}),
    3 => Box::new(Puzzle3 {}),
    4 => Box::new(Puzzle4 {}),
    5 => Box::new(Puzzle5 {}),
    6 => Box::new(Puzzle6 {}),
    _ => Box::new(Puzzle7 {}),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_generates_8_characters() {
    let equation = equation_generator().gen_equation();
    println!("{}", equation);
    assert_eq!(equation.chars().count(), 8);
  }
}
