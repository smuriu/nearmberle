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
  <button class="btn btn-outline basis-1/5" type="button" :disabled="disabled" :class="busy ? 'loading' : ''"
    @click.stop="emit('kbd_input', value)">{{
        value
    }}</button>
</template>
