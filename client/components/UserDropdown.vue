<script setup lang="ts">
const { getAccountId, signOut } = useNearAuth()

const playerId = computed(() => getAccountId())
const initials = computed(() => {
  const parts = playerId.value.split('.')
  return parts.map((part) => part.substring(0, 1)).join().toUpperCase()
})
</script>

<template>
  <div class="dropdown dropdown-end">
    <label tabindex="0" class="btn btn-circle m-1">{{ initials }}</label>
    <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
      <li>{{ playerId }}</li>
      <li><a @click="signOut()">Logout</a></li>
    </ul>
  </div>
</template>
