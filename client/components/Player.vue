<script lang="ts" setup>
const { getAccountId, signOut } = useNearAuth()
const { getStats } = useNearmberle()

const { data: stats, refresh, pending } = await useAsyncData('player', async () => {
  const stats = await getStats()
  return stats
})

const playerId = computed(() => getAccountId())
const play = () => {
  useRouter().push('/game')
}
</script>

<template>
  <div class="card bg-neutral text-neutral-content">
    <div class="card-body">
      <h2 class="card-title">
        {{ playerId }}
        <button class="badge badge-outline" :aria-busy="pending" @click="refresh()">🔄</button>
      </h2>
      <pre><code>{{ stats }}</code></pre>
      <div class="card-actions justify-end">
        <button class="btn btn-primary" @click="play">Play</button>
        <button class="btn btn-ghost" @click="signOut">Logout</button>
      </div>
    </div>
  </div>
</template>
