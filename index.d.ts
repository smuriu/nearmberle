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

declare type PlayerStats = {
  played: number
  solved: number
  streak: number
}

declare type GameState = {
  puzzleId: string
  attempts: [submission: string, result: PuzzleStatus][]
}
