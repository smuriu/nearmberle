<script setup lang="ts">
const { busy, state, attempt, start } = useNearmberle()

const lastResult = computed(() => {
  const { attempts } = state.value
  const count = attempts.length
  if (count > 0) {
    const [submission, result] = attempts[count - 1]
    return result
  }

  return null
})

// TODO: add auth middleware
// challenge: presently, auth is only on browser-side
</script>

<template>
  <div v-if="state">
    <h1>Game in progress</h1>
    <details>
      <summary><strong>Rules of the game</strong></summary>
      <ul>
        <li>Your job is to try to guess an 8 character equation.</li>
        <li>Green means the right character/operator in the right place.</li>
        <li>Yellow means the right character/operator in the wrong place.</li>
        <li>Grey means an entirely wrong character/operator.</li>
      </ul>
    </details>
    <section>
      <GameBoard :state="state">
        <div v-if="lastResult?.Failed">
          You failed. Solution was {{ lastResult.Failed.soln }}
          <button :aria-busy="busy" @click="start">New Game</button>
        </div>
        <div v-else-if="lastResult?.Solved">
          Congratulations! You guessed right in {{ lastResult.Solved.attempts }} attempts
          <button :aria-busy="busy" @click="start">New Game</button>
        </div>
        <GuessForm v-else @guess="attempt" />
      </GameBoard>
    </section>
  </div>
  <div v-else>
    <button :aria-busy="busy" @click="start">New Game</button>
  </div>
</template>
