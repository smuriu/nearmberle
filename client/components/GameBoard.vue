<script setup lang="ts">
defineProps({
  state: Object as () => GameState,
})
const hintMap = {
  0: 'bg-gray-600 text-white',
  1: 'bg-yellow-400 text-black',
  2: 'bg-green-700 text-white',
}
const description = {
  0: 'is not in the word.',
  1: 'is in the word, but not at the location you picked.',
  2: 'is in exactly the right location',
}
const letterClasses = (result: PuzzleStatus, index: number): string => {
  let classes = ''
  if (result.Playing) {
    classes = hintMap[result.Playing.hint[index]]
  }

  return classes
}
const letterText = (result: PuzzleStatus, index: number): string => {
  let text = ''
  if (result.Playing) {
    text = description[result.Playing.hint[index]]
  }

  return text
}
</script>

<template>
  <article>
    <header>Game #{{ state.puzzleId }}</header>
    <ul>
      <li v-for="[submission, result] in state.attempts" class="flex gap-1">
        <span v-for="(letter, i) in submission" class="block w-6 h-6 text-center font-bold uppercase"
          :class="letterClasses(result, i)">
          {{ letter }}
          <span class="sr-only">
            {{ letterText(result, i) }}
          </span>
        </span>
      </li>
    </ul>
    <footer>
      <slot />
    </footer>
  </article>
</template>
