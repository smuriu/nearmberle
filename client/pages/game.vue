<script setup lang="ts">
const { state, attempt, start } = useNearmberle()

const lastResult = computed(() => {
  const { attempts } = state.value
  const count = attempts.length
  if (count > 0) {
    const [submission, result] = attempts[count - 1]
    return result
  }

  return null
})
</script>

<template>
  <div>
    <h1>Play</h1>
    <details>
      <summary><strong>Rules of the game</strong></summary>
      <ul>
        <li>Your job is to try to guess an 8 character equation.</li>
        <li>Green means the right digit/operator in the right place.</li>
        <li>Yellow means the right digit/operator in the wrong position.</li>
        <li>Grey means an entirely wrong digit/operator.</li>
      </ul>
    </details>
    <section>
      <ClientOnly>
        <WalletComponent>
          <GameBoard v-if="state" :state="state">
            <div v-if="lastResult?.Failed">
              You failed. Solution was {{ lastResult.Failed.soln }}
              <NewGameButton @start="start()" />
            </div>
            <div v-else-if="lastResult?.Solved">
              Congratulations! You guessed right in {{ lastResult.Solved.attempts }} attempts
              <NewGameButton @start="start()" />
            </div>
            <GuessForm v-else @guess="attempt" />
          </GameBoard>
          <div v-else>
            <NewGameButton @start="start()" />
          </div>
        </WalletComponent>
      </ClientOnly>
    </section>
  </div>
</template>
