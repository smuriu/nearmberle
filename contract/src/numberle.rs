use crate::generator::equation_generator;
use near_sdk::{
  borsh::{self, *},
  env,
  serde::{Deserialize, Serialize},
  AccountId,
};

const PUZZLE_LENGTH: usize = 8;
const MAX_ATTEMPTS: u8 = 6;

fn score(submission: &str, equation: &str) -> [u8; PUZZLE_LENGTH] {
  assert_eq!(submission.len(), PUZZLE_LENGTH);
  let mut scores: [u8; PUZZLE_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0];
  submission.char_indices().for_each(|(i, x)| {
    let mut val: u8 = 0;
    if equation.find(x) != None {
      val += 1;
    }

    if equation.char_indices().any(|(n, y)| n == i && y == x) {
      val += 1;
    }

    scores[i] = val;
  });

  scores
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PuzzleStatus {
  Playing {
    attempts: u8,
    hint: [u8; PUZZLE_LENGTH],
  },
  Solved {
    attempts: u8,
  },
  Failed {
    attempts: u8,
    soln: String,
  },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Numberle {
  player_id: AccountId,
  equation: String,
  status: PuzzleStatus,
}

impl Numberle {
  pub fn new() -> Self {
    let equation = equation_generator().gen_equation();
    let hint: [u8; PUZZLE_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0];
    Self {
      player_id: env::predecessor_account_id(),
      equation,
      status: PuzzleStatus::Playing { attempts: 0, hint },
    }
  }

  pub fn attempt(&mut self, submission: &str) -> Option<PuzzleStatus> {
    assert_eq!(
      self.player_id,
      env::predecessor_account_id(),
      "Only you can play your game"
    );

    if let PuzzleStatus::Playing { attempts, hint: _ } = &self.status {
      let new_attempts = attempts + 1;
      let scores = score(submission, self.equation.as_str());
      let mut total: u8 = 0;
      scores.iter().for_each(|x| total += x);
      if (2 * PUZZLE_LENGTH) > total.into() {
        if new_attempts < MAX_ATTEMPTS {
          self.status = PuzzleStatus::Playing {
            attempts: new_attempts,
            hint: scores,
          };
        } else {
          self.status = PuzzleStatus::Failed {
            attempts: new_attempts,
            soln: self.equation.clone(),
          };
        }
      } else {
        self.status = PuzzleStatus::Solved {
          attempts: new_attempts,
        };
      }
      return Some(self.status.clone());
    }

    None
  }

  pub fn get_status(&self) -> PuzzleStatus {
    self.status.clone()
  }
}

#[cfg(test)]
mod tests {
  // use crate::numberle::Numberle;
  use super::*;

  #[test]
  fn attempt() {
    let mut puzzle = Numberle::new();
    println!("{:?}", puzzle.attempt("12*9=108"));
    println!("{:?}", puzzle.attempt("5*6+9=39"));
    println!("{:?}", puzzle.attempt("12*9=108"));
    println!("{:?}", puzzle.attempt("99+9=108"));
    println!("{:?}", puzzle.attempt("9*9/3=27"));
    println!("{:?}", puzzle.attempt("12*9=108"));
  }
}
