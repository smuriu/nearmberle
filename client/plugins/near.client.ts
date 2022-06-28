import { connect, keyStores, Near, WalletConnection } from "near-api-js"

// @see https://github.com/near/borsh-js/issues/48
// and https://github.com/near/borsh-js/pull/50
import { Buffer } from "buffer"
globalThis.Buffer = Buffer

// reference to process.env in https://github.com/near/near-api-js/blob/9e737e325c4b28ca59fc6245351be3b8ea54fe92/lib/account.js#L78
// but not available in browser https://github.com/nuxt/framework/issues/2797
// TODO: Find better way to polyfill `process` in browser
if (typeof globalThis.process == "undefined") {
  // @ts-ignore
  globalThis.process = {
    env: {}
  }
} else {
  globalThis.process.env = {}
}

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

  provide('near', near)
  provide('wallet', wallet)
})

declare module '#app' {
  interface NuxtApp {
    $near: Near
    $wallet: WalletConnection
  }
}

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $near: Near
    $wallet: WalletConnection
  }
}
