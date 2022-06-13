import { defineNuxtConfig } from 'nuxt'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  runtimeConfig: {
    public: {
      near: {
        network: 'testnet',
        appTitle: 'NEARmberle',
        contractName: 'nearmberle.sosybuntu.testnet'
      }
    }
  },
  // ssr: false
})
