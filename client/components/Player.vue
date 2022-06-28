<script lang="ts" setup>
const { getAccountId, isSignedIn, signIn, signOut } = useNearAuth()
const { getStats } = useNearmberle()

const { data: stats } = await useAsyncData('player', async () => {
  const stats = await getStats()
  return stats
})

const playerId = computed(() => getAccountId())
const play = () => {
  useRouter().push('/game')
}
</script>

<template>
  <article v-if="isSignedIn()">
    <header>{{ playerId }}</header>
    <pre>
      <code>{{ stats }}</code>
    </pre>
    <footer class="grid">
      <button @click="play">Play</button>
      <button @click="signOut">Logout</button>
    </footer>
  </article>
  <article v-else>
    <button @click="signIn">Login</button>
  </article>
</template>
