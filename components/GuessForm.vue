<script setup lang="ts">
import { useBusy } from '~~/composables/busy'

const emit = defineEmits<{
  (event: 'guess', guess: string): void
}>()

const busy = useBusy()
const guess = ref('')

const submitGuess = () => {
  if (!busy.value) {
    emit('guess', guess.value)
  }
}
</script>

<template>
  <form @submit.prevent="submitGuess">
    <label>
      Your guess
      <input v-model="guess" type="text" minlength="8" maxlength="8"
        class="font-bold uppercase tracking-widest text-black bg-white" />
      <button :disabled="guess.length !== 8" :aria-busy="busy" type="submit">Guess</button>
    </label>
  </form>
</template>
