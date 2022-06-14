import { joinURL } from "ufo"

export const useNearAuth = () => {
  const { appTitle, contractName } = useRuntimeConfig().public.near

  const isSignedIn = () => {
    const accountId = useCookie('nmbl_account')
    return !!accountId.value
  }

  function signIn() {
    const { $wallet } = useNuxtApp()
    const router = useRouter()
    const host = useHost()
    const { href } = router.resolve('/auth/success')
    const successUrl = joinURL(host, href)
    $wallet.requestSignIn(contractName, appTitle, successUrl)
  }

  async function signOut() {
    const { $wallet } = useNuxtApp()
    $wallet.signOut()
    await $fetch('/auth/logout')
    window.location.reload()
  }

  return { isSignedIn, signIn, signOut }
}
