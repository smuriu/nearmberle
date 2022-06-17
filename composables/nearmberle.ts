import { Contract } from "near-api-js"
import { useBusy } from "./busy"

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

  const toggleBusy = () => {
    const busy = useBusy()
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
    const contract = loadContract()
    const player_id = getAccountId()

    // @ts-ignore
    const stats = await contract.stats_by_player({ player_id })

    return stats || {
      played: 0,
      solved: 0,
      streak: 0
    }
  }

  return { state, start, attempt, getStats }
}
