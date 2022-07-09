<script setup lang="ts">
const emit = defineEmits<{
  (event: 'guess', guess: string): void
}>()

const { busy } = useNearmberle()
const guess = ref('')
const kbdInputDisabled = computed(() => guess.value.length > 7)

const submitGuess = () => {
  if (!busy.value) {
    emit('guess', guess.value)
    guess.value = ''
  }
}

const onKbdInput = (value: string) => {
  if (guess.value.length < 8) {
    guess.value += value
  }
}

const kbdBackspace = () => {
  guess.value = guess.value.replace(/.$/, '')
}
</script>

<template>
  <form @submit.prevent="submitGuess">
    <div class="flex flex-col">
      <div class="form-control w-full mb-2">
        <label class="label">
          <span class="label-text">Your guess</span>
        </label>
        <input v-model="guess" type="text" minlength="8" maxlength="8" placeholder="eg. 12+34=46"
          class="input input-bordered input-lg w-full" />
      </div>

      <div class="flex flex-col">
        <div class="flex justify-center gap-1 my-1 w-full">
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="1" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="2" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="3" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="4" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="5" @kbd_input="onKbdInput" />
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="6" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="7" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="8" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="9" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="0" @kbd_input="onKbdInput" />
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="'+'" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="'-'" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="'*'" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="'/'" @kbd_input="onKbdInput" />
          <KbdButton :disabled="kbdInputDisabled" :busy="busy" :value="'='" @kbd_input="onKbdInput" />
        </div>
        <div class="flex justify-center gap-1 my-1 w-full">
          <button :disabled="guess.length === 0" class="btn btn-outline basis-1/3" :class="busy ? 'loading' : ''"
            @click.stop="kbdBackspace" type="button">⇐</button>
          <button :disabled="guess.length !== 8" type="submit" class="btn btn-outline btn-primary basis-2/3"
            :class="busy ? 'loading' : ''">Guess</button>
        </div>
      </div>
    </div>
  </form>
</template>
