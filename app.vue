<template>
  <div class="container">
    <Html lang="en-GB" />
    <Title>NEARmberle</Title>
    <Meta name="description" content="A tiny, incomplete clone of n3dle built on NEAR as a bootcamp assignment." />
    <!-- <Script src="https://cdn.jsdelivr.net/gh/nearprotocol/near-api-js/dist/near-api-js.js" /> -->
    <header>
      <nav>
        <ul>
          <li>
            <NuxtLink to="/" class="flex items-center gap-2 text-green-400">
              <img alt="Nuxt logo" src="https://nuxtjs.org/design-kit/colored-logo.svg" height="24" width="24" />
              <strong>NEARmberle</strong>
            </NuxtLink>
          </li>
        </ul>
      </nav>
    </header>
    <main>
      <NuxtPage />
    </main>
    <footer>
      <small>
        Made with ❤️ &middot;
        <a href="https://github.com/smuriu/nearmberle">Contributions welcome</a>
      </small>
    </footer>
  </div>
</template>

<style>
/* Blue-grey Light scheme (Default) */
/* Can be forced with data-theme="light" */
[data-theme='light'],
:root:not([data-theme='dark']) {
  --primary: #546e7a;
  --primary-hover: #455a64;
  --primary-focus: rgba(84, 110, 122, 0.125);
  --primary-inverse: #fff;
}

/* Blue-grey Dark scheme (Auto) */
/* Automatically enabled if user has Dark mode enabled */
@media only screen and (prefers-color-scheme: dark) {
  :root:not([data-theme='light']) {
    --primary: #546e7a;
    --primary-hover: #607d8b;
    --primary-focus: rgba(84, 110, 122, 0.25);
    --primary-inverse: #fff;
  }
}

/* Blue-grey Dark scheme (Forced) */
/* Enabled if forced with data-theme="dark" */
[data-theme='dark'] {
  --primary: #546e7a;
  --primary-hover: #607d8b;
  --primary-focus: rgba(84, 110, 122, 0.25);
  --primary-inverse: #fff;
}

/* Blue-grey (Common styles) */
:root {
  --form-element-active-border-color: var(--primary);
  --form-element-focus-color: var(--primary-focus);
  --switch-color: var(--primary-inverse);
  --switch-checked-background-color: var(--primary);
}
</style>
