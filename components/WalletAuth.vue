<script lang="ts" setup>
const { signIn, signOut } = useNearmberle()

const loggedIn = computed(() => {
  const { $wallet } = useNuxtApp()
  return $wallet.isSignedIn()
})
</script>

<template>
  <li>
    <button v-if="loggedIn" @click="signOut()">Logout</button>
    <button v-else @click="signIn()">Log in</button>
  </li>
</template>
