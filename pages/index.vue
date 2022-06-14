<script setup lang="ts">
const { isSignedIn } = useNearAuth()
</script>

<template>
  <section>
    <h1>Welcome to NEARmberle</h1>
    <p>
      This is a tiny, incomplete clone of
      <a href="https://github.com/danielroe/n3rdle">n3dle</a> built in
      <a href="https://rust-lang.org/">Rust</a> for a bootcamp assignment for
      <a href="https://near.org/">NEAR</a>.
    </p>
    <NuxtLink v-if="isSignedIn()" to="/game" role="button">Play game</NuxtLink>
  </section>
</template>
