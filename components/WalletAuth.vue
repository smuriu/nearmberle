<script lang="ts" setup>
const { isSignedIn, signIn, signOut } = useNearAuth()
</script>

<template>
  <li>
    <button v-if="isSignedIn()" @click="signOut()">Logout</button>
    <button v-else @click="signIn()">Log in</button>
  </li>
</template>
