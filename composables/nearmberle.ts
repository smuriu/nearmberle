import { Contract } from "near-api-js"

const loadContract = () => {
  const { contractName } = useRuntimeConfig().public.near
  const { $wallet } = useNuxtApp()

  if (!$wallet.isSignedIn()) {
    throwError('unauthenticated')
  }

  return new Contract(
    $wallet.account(),
    contractName,
    {
      viewMethods: ["stats_by_player"], // view methods do not change state but usually return a value
      changeMethods: ["hit_me", "attempt"], // change methods modify state
    }
  )
}

const getAccountId = () => {
  const { $wallet } = useNuxtApp()
  return $wallet.getAccountId()
}

const decode = (state: string): GameState => JSON.parse(state)
const encode = (state: GameState): string => JSON.stringify(state)

export const useNearmberle = () => {

  const state = useCookie<GameState>('nmbl_state', {
    decode,
    encode
  })

  const busy = useState<boolean>('nmbl_busy', () => false)

  const toggleBusy = () => {
    busy.value = !busy.value
  }

  async function start() {
    toggleBusy()

    const contract = loadContract()
    // @ts-ignore
    const puzzleId: string = await contract.hit_me({})

    state.value = {
      puzzleId,
      attempts: []
    }

    toggleBusy()
  }

  async function attempt(submission: string) {
    // TODO: add error-handling e.g. if playing non-existant game or not own game
    toggleBusy()

    const contract = loadContract()
    const { puzzleId, attempts } = state.value

    // @ts-ignore
    const result: PuzzleStatus = await contract.attempt({
      game_id: puzzleId,
      submission
    })
    attempts.push([submission, result])

    state.value = {
      puzzleId,
      attempts
    }

    toggleBusy()
  }

  async function getStats(): Promise<PlayerStats> {
    let stats = {
      played: 0,
      solved: 0,
      streak: 0
    }

    const { $wallet } = useNuxtApp()
    if ($wallet.isSignedIn()) {
      const contract = loadContract()
      const player_id = getAccountId()

      // @ts-ignore
      const result = await contract.stats_by_player({ player_id })
      if (result) {
        stats = result
      }
    }

    return stats 
  }

  return { busy, state, start, attempt, getStats }
}
