import { Contract } from "near-api-js"

export const useNearmberle = () => {
  const { contractName } = useRuntimeConfig().public.near

  function loadContract(): Contract {
    const { $wallet } = useNuxtApp()
    return new Contract(
      $wallet.account(),
      contractName,
      {
        viewMethods: ["stats_by_player"], // view methods do not change state but usually return a value
        changeMethods: ["hit_me", "attempt"], // change methods modify state
      }
    )
  }

  async function start() {
    const contract = loadContract()
    // @ts-ignore
    const response = await contract.hit_me()
    console.log(response)
  }

  async function attempt(submission: string) {
    const contract = loadContract()
    // @ts-ignore
    const response = await contract.attempt({
      game_id: '',
      submission
    })
    console.log(response)
  }

  async function getStats() {
    const contract = loadContract()
    // @ts-ignore
    const response = await contract.stats_by_player({ player_id: "sosybuntu.testnet" })
    console.log(response)
  }

  return { start, attempt, getStats }
}
