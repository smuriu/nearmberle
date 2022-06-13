import { Contract } from "near-api-js"

export const useNearmberle = () => {
  const { appTitle, contractName } = useRuntimeConfig().public.near

  // () => useState('x')
  function isSignedIn(): boolean {
    const { $wallet } = useNuxtApp()
    return $wallet.isSignedIn()
  }

  function signOut() {
    const { $wallet } = useNuxtApp()
    $wallet.signOut()
  }

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

  async function signIn() {
    const { $wallet } = useNuxtApp()
    await $wallet.requestSignIn(contractName, appTitle)
    // ?account_id=sosybuntu.testnet&public_key=ed25519:EieDHU1mKeh746rjKxYFF1E1gUaowZyBCBWJoyNVbQmB&all_keys=ed25519:9wzREQGTcnUi7dyAfbB8Qh1rkXKVPEDWNSTEs7Jy1qjb
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

  return { isSignedIn, signIn, signOut, start, attempt, getStats }
}
