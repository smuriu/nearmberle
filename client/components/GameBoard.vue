<script setup lang="ts">
defineProps({
  state: Object as () => GameState,
})
const hintMap = {
  0: 'bg-neutral text-neutral-content',
  1: 'bg-warning text-warning-content',
  2: 'bg-success text-success-content',
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
const letterText = (submission: string, result: PuzzleStatus, index: number): string => {
  let text = ''
  if (result.Playing) {
    text = `${submission[index]} ${description[result.Playing.hint[index]]}`
  }

  return text
}
</script>

<template>
  <div class="card bg-neutral text-neutral-content shadow-xl">
    <div class="card-body items-center text-center">
      <h2 class="card-title">Game #{{ state.puzzleId }}</h2>
      <div v-if="state.attempts.length > 0" class="flex flex-col gap-1">
        <div v-for="[submission, result] in state.attempts" class="flex gap-1">
          <div v-for="(letter, i) in submission" class="tooltip" :data-tip="letterText(submission, result, i)">
            <kbd class="kbd" :class="letterClasses(result, i)">
              {{ letter }}
            </kbd>
          </div>
        </div>
      </div>
      <div v-else>
        <span class="italic">Make your first guess</span>
      </div>
      <div class="card-actions">
        <slot />
      </div>
    </div>
  </div>
</template>
