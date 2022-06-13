use crate::numberle::{Numberle, PuzzleStatus};
use near_sdk::borsh::{self, *};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::*;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
  GAME,
  STAT,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerStats {
  played: usize,
  solved: usize,
  streak: usize,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  active_games: LookupMap<String, Numberle>,
  player_stats: UnorderedMap<String, PlayerStats>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      active_games: LookupMap::new(StorageKey::GAME),
      player_stats: UnorderedMap::new(StorageKey::STAT),
    }
  }

  // TODO: make payable?
  // at least enough for projected lifetime storage costs for stats,
  // storage costs for active games
  // TODO: also consider tournaments with a winner's pot
  pub fn hit_me(&mut self) -> String {
    let game_id = env::block_timestamp().to_string();
    let puzzle = Numberle::new();
    self.active_games.insert(&game_id, &puzzle);
    game_id.clone()
  }

  fn update_stats(&mut self, puzzle_status: &PuzzleStatus) {
    if let PuzzleStatus::Playing {
      attempts: _,
      hint: _,
    } = puzzle_status
    {
      return;
    }

    let player = env::predecessor_account_id().to_string();
    let mut stats = match self.player_stats.get(&player) {
      Some(prior_stats) => prior_stats,
      _ => PlayerStats::default(),
    };
    stats.played += 1;

    if let PuzzleStatus::Solved { attempts: _ } = puzzle_status {
      stats.solved += 1;
      stats.streak += 1;
    } else {
      // PuzzleStatus::Failed
      stats.streak = 0;
    }
    self.player_stats.insert(&player, &stats);
  }

  pub fn attempt(&mut self, game_id: String, submission: String) -> Option<PuzzleStatus> {
    let key = &game_id;
    assert!(self.active_games.contains_key(key), "Game 404");
    let mut game = self.active_games.get(key).unwrap();
    if let Some(status) = game.attempt(submission.as_str()) {
      self.update_stats(&status);
      if let PuzzleStatus::Playing {
        attempts: _,
        hint: _,
      } = status
      {
        // update storage
        self.active_games.insert(key, &game);
      } else {
        self.active_games.remove(key);
      }
      return Some(status);
    }
    None
  }

  pub fn stats_by_player(&self, player_id: String) -> Option<PlayerStats> {
    self.player_stats.get(&player_id)
  }
}
