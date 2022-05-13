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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
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

mod tests {
  use super::*;

  #[test]
  fn attempt_on_active_game_produces_some_status() {
    let mut game = Numberle::new();
    let status = game.attempt("10/0=NaN");
    assert_ne!(status, None);
  }

  #[test]
  fn no_more_than_6_attempts_per_game() {
    let mut game = Numberle::new();
    game.attempt("11/0=NaN");
    game.attempt("12/0=NaN");
    game.attempt("13/0=NaN");
    game.attempt("14/0=NaN");
    game.attempt("15/0=NaN");
    game.attempt("16/0=NaN");
    let status = game.attempt("10/0=NaN");
    assert_eq!(status, None);
  }
}
