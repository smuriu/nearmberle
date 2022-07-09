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
    <div class="container mx-auto p-4 bg-neutral text-neutral-content">
      <h1 class="text-2xl font-bold">Play</h1>
      <div tabindex="0" class="collapse collapse-arrow my-4 p-4">
        <div class="collapse-title text-xl font-medium">
          Rules of the game
        </div>
        <div class="collapse-content">
          <ul>
            <li>Your job is to try to guess an 8 character equation.</li>
            <li>Green means the right digit/operator in the right place.</li>
            <li>Yellow means the right digit/operator in the wrong position.</li>
            <li>Grey means an entirely wrong digit/operator.</li>
          </ul>
        </div>
      </div>
      <div>
        <ClientOnly>
          <WalletComponent>
            <GameBoard v-if="state" :state="state">
              <div v-if="lastResult?.Failed">
                <p>You failed. Solution was {{ lastResult.Failed.soln }}</p>
                <NewGameButton @start="start()" />
                <NuxtLink class="btn" :to="{ path: '/' }">Stats</NuxtLink>
              </div>
              <div v-else-if="lastResult?.Solved">
                <p>Congratulations! You guessed right in {{ lastResult.Solved.attempts }} attempts</p>
                <NewGameButton @start="start()" />
                <NuxtLink class="btn" :to="{ path: '/' }">Stats</NuxtLink>
              </div>
              <GuessForm v-else @guess="attempt" />
            </GameBoard>
            <div v-else>
              <NewGameButton @start="start()" />
            </div>
          </WalletComponent>
        </ClientOnly>
      </div>
    </div>
  </div>
</template>
