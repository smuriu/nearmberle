<template>
  <div>
    <Html lang="en-GB" />
    <Title>NEARmberle</Title>
    <Meta name="description" content="A tiny, incomplete clone of n3dle built on NEAR as a bootcamp assignment." />
    <div class="navbar">
      <div class="navbar-start">
        <NuxtLink class="btn btn-link" :to="{ path: '/' }">Home</NuxtLink>
        <NuxtLink class="btn btn-link" :to="{ path: '/game' }">Game</NuxtLink>
      </div>
      <div class="navbar-center">
        <NuxtLink to="/" class="flex items-center gap-2 text-green-400">
          <img alt="Nuxt logo" src="https://nuxtjs.org/design-kit/colored-logo.svg" height="24" width="24" />
          <strong>NEARmberle</strong>
        </NuxtLink>
      </div>
      <div class="navbar-end">
        <ClientOnly>
          <WalletComponent>
            <UserDropdown />
          </WalletComponent>
        </ClientOnly>
      </div>
    </div>
    <NuxtPage />
    <footer class="footer footer-center p-4 bg-neutral text-neutral-content">
      <div>
        <p>Made with ❤️ &middot; <a href="https://github.com/smuriu/nearmberle">Contributions welcome</a></p>
      </div>
    </footer>
  </div>
</template>
