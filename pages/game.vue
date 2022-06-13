<script setup lang="ts">
const { state, error, submitGuess, resetGame } = useGameState()
</script>

<template>
  <div>
    <h1>Game in progress</h1>
    <details>
      <summary><strong>Rules of the game</strong></summary>
      <ul>
        <li>Your job is to try to guess an 8 character equation.</li>
        <li>Green means the right character/operator in the right place.</li>
        <li>Yellow means the right  character/operator in the wrong place.</li>
        <li>Grey means an entirely wrong  character/operator.</li>
      </ul>
    </details>
    <section>
      <GameBoard :state="state" />
      <GuessError :error="error" />
      <GuessForm @guess="submitGuess" />
      <button class="secondary outline" @click="resetGame">Reset game</button>
    </section>
  </div>
</template>
