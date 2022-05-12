/* use near_sdk::{
  borsh::{self, *},
  collections::*,
  json_types::*,
  serde::{self, *},
  *,
}; */
mod generator;
mod numberle;
// mod utils;

mod contract;
pub use contract::*;

#[cfg(test)]
mod tests {
  use crate::*;
  use near_sdk::test_utils::*;
  use near_sdk::{testing_env, AccountId};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test() {
    let context = get_context(contract_account());
    testing_env!(context.build());
    let mut game = Contract::new();

    let game_id = game.hit_me();

    for submission in [
      "99*2=198", "12*9=108", "99+9=108", "5*6+9=39", "108/12=9",
      "9*9/3=27",
      // 67-14=53, 19-7-5=7
    ] {
      if let Some(status) = game.attempt(game_id.clone(), submission.to_string()) {
        println!("{}, {}, {:?}", &game_id, submission, status);
        match status {
          numberle::PuzzleStatus::Playing {
            attempts: _,
            hint: _,
          } => {}
          _ => break,
        };
      } else {
        break;
      }
    }

    println!("{:?}", game.get_stats());
  }
}
