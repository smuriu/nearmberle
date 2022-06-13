import { connect, keyStores, WalletConnection } from "near-api-js"

// @see https://github.com/near/borsh-js/issues/48
// and https://github.com/near/borsh-js/pull/50
import { Buffer } from "buffer"
globalThis.Buffer = Buffer

export default defineNuxtPlugin(async ({ provide }) => {
  const { network } = useRuntimeConfig().public.near
  const keyStore = new keyStores.BrowserLocalStorageKeyStore()
  const config = {
    networkId: network,
    keyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
    headers: {}
  }

  const near = await connect(config)
  const wallet = new WalletConnection(near, 'nmbl')

  provide('wallet', wallet)
})

declare module '#app' {
  interface NuxtApp {
    $wallet: WalletConnection
  }
}

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $wallet: WalletConnection
  }
}
