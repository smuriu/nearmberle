<script setup lang="ts">
defineProps<{
  disabled: boolean
  busy: boolean
  value: string | number
}>()

const emit = defineEmits<{
  (event: 'kbd_input', value: string | number): void
}>()
</script>

<template>
  <button :disabled="disabled" :aria-busy="busy" @click.stop="emit('kbd_input', value)" class="kbd" type="button">{{
      value
  }}</button>
</template>
