// import { joinURL } from "ufo"

export const useNearAuth = () => {
  const { appTitle, contractName } = useRuntimeConfig().public.near

  const isSignedIn = () => {
    const { $wallet } = useNuxtApp()
    return $wallet.isSignedIn()
  }

  const getAccountId = (): string => {
    const { $wallet } = useNuxtApp()
    return $wallet.getAccountId()
  }

  function signIn() {
    const { $wallet } = useNuxtApp()
    /* const host = useHost()
    const { href } = useRouter().resolve('/auth/success')
    const successUrl = joinURL(host, href) */

    $wallet.requestSignIn(contractName, appTitle/* , successUrl */)
  }

  async function signOut() {
    const { $wallet } = useNuxtApp()
    const { state } = useNearmberle()

    // abandons current game
    state.value = null
    $wallet.signOut()

    /* useRouter().push({
      path: '/',
      force: true
    }) */
    window.location.reload()
  }

  return { getAccountId, isSignedIn, signIn, signOut }
}
