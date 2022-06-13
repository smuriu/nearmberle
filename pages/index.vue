<script setup lang="ts">
const { isSignedIn, getStats } = useNearmberle()

if (isSignedIn) {
  getStats()
}
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
    <NuxtLink to="/game" role="button">Play game</NuxtLink>
  </section>
</template>
