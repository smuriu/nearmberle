<script setup lang="ts">
const emit = defineEmits<{
  (event: 'start'): void
}>()
const { busy } = useNearmberle()
</script>

<template>
  <button class="btn btn-primary" :class="busy ? 'loading' : ''" @click="emit('start')">New Game</button>
</template>
