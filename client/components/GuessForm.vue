<script setup lang="ts">
const emit = defineEmits<{
  (event: 'guess', guess: string): void
}>()

const { busy } = useNearmberle()
const guess = ref('')

const submitGuess = () => {
  if (!busy.value) {
    emit('guess', guess.value)
    guess.value = ''
  }
}

const kbdClick = ({ target }) => {
  if (guess.value.length < 8) {
    guess.value += target.innerText
  }
}

const kbdBackspace = () => {
  guess.value = guess.value.replace(/.$/, '')
}
</script>

<template>
  <form @submit.prevent="submitGuess">
    <div class="flex flex-col">
      <label>
        Your guess
        <input v-model="guess" type="text" minlength="8" maxlength="8"
          class="font-bold uppercase tracking-widest text-black bg-white" />
      </label>
      <div class="flex flex-col">
        <div class="flex justify-center gap-1 my-1 w-full">
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">1</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">2</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">3</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">4</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">5</button>
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">6</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">7</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">8</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">9</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">0</button>
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">+</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">-</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">*</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">/</button>
          <button :disabled="guess.length > 7 || busy" class="kbd" @click.stop="kbdClick" type="button">=</button>
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <button :disabled="guess.length === 0" class="kbd" @click.stop="kbdBackspace" type="button">⇐</button>
          <button :disabled="guess.length !== 8" :aria-busy="busy" type="submit" class="kbd">Guess</button>
        </div>
      </div>
    </div>
  </form>
</template>
