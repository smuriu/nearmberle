declare type PuzzleStatus = {
  Playing?: {
    attempts: number,
    hint: number[]
  },
  Failed?: {
    attempts: number,
    soln: string
  },
  Solved?: {
    attempts: number
  }
}

declare type GameState = [guess: string, hint: string][]
