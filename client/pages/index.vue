<template>
  <div class="container mx-auto p-4 bg-neutral text-neutral-content">
    <h1 class="text-2xl font-bold">Welcome to NEARmberle</h1>
    <p>
      This is a tiny, incomplete clone of
      <a href="https://github.com/danielroe/n3rdle">n3dle</a> built in
      <a href="https://rust-lang.org/">Rust</a> and
      <a href="https://v3.nuxtjs.org">Nuxt 3</a> for a bootcamp assignment for
      <a href="https://near.org/">NEAR protocol</a>.
    </p>
    <ClientOnly>
      <WalletComponent>
        <Player />
      </WalletComponent>
    </ClientOnly>
  </div>
</template>
