<script setup lang="ts">
const { isSignedIn, signIn } = useNearAuth()
</script>

<template>
  <div v-if="isSignedIn()">
    <slot />
  </div>
  <div v-else>
    <button class="btn" @click="signIn()">Login</button>
  </div>
</template>
