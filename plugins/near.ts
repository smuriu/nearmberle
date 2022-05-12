import VueNear from "vue-near"

export default defineNuxtPlugin(({ vueApp }) => {
  vueApp.use(VueNear, {
    // Needs the environment for the correct RPC to use
    env: process.env.NODE_ENV || 'development',
    config: {
      appTite: 'Nearmberle',
      contractName: 'nearmberle.sosybuntu.testnet',
    },
  })
})
