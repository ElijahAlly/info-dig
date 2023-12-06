<template>
  <nav>
    <router-link class="img-container" to="/">
      <img alt="info-dig logo" class="app-logo" src="./assets/info-dig-logo-transparent.png" width="60px" />
    </router-link>
    <router-link class="nav-link" to="/">Home</router-link> |
    <router-link class="nav-link" to="/statements">Statements</router-link> |
    <router-link class="nav-link" to="/proposals">Proposals</router-link> |
    <router-link class="nav-link" to="/campaigns">Campaigns</router-link> |
    <router-link class="nav-link" to="/research">Research</router-link> |
    <router-link class="nav-link" to="/about">About</router-link>
  </nav>
  <router-view />
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

nav {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 12px;

  .nav-link {
    font-weight: bold;
    color: #2c3e50;
    margin: 0 1em;    
    transition: 0.3s ease-in-out;

    &.router-link-exact-active {
      color: #42b983;
    }

    &:hover {
      text-shadow: #2c3e50;
    }
  }

  .app-logo {
    border-radius: 50%;
    margin-right: 1em;
  }

  .img-container {
    cursor: pointer;
    clip-path: circle();
    width: 60px;
    height: 60px;
    transition: 1s ease-in-out;

    &:hover {
      rotate: -360deg 1 2 1;
      opacity: 81%;
      filter: blur(1px);
    }
  }
}
</style>
