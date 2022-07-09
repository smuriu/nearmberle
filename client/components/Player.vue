<script lang="ts" setup>
const { getAccountId, signOut } = useNearAuth()
const { getStats } = useNearmberle()

const { data: stats, refresh, pending } = await useAsyncData('player', async () => {
  const stats = await getStats()
  return stats
})

const playerId = computed(() => getAccountId())
</script>

<template>
  <div class="card bg-neutral text-neutral-content">
    <div class="card-body">
      <div class="flex justify-between">
        <span class="card-title">{{ playerId }}</span>
        <div>
          <button class="btn btn-sm btn-outline" :class="pending ? 'loading' : ''" @click="refresh()">⟳</button>
        </div>
      </div>
      <pre><code>{{ stats }}</code></pre>
      <div class="card-actions justify-end">
        <NuxtLink class="btn btn-primary" :to="{
          path: '/game'
        }">Play</NuxtLink>
      </div>
    </div>
  </div>
</template>
