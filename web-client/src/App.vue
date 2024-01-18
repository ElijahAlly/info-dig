<template>
  <nav>
    <router-link class="img-container" to="/">
      <img alt="info-dig logo" class="app-logo" src="./assets/logos/info-dig-logo-transparent.png" width="60px" />
    </router-link>
    <router-link class="nav-link" to="/">Home</router-link> |
    <router-link class="nav-link" to="/statements">Statements</router-link> |
    <router-link class="nav-link" to="/proposals">Proposals</router-link> |
    <router-link class="nav-link" to="/campaigns">Campaigns</router-link> |
    <router-link class="nav-link" to="/research">Research</router-link> |
    <router-link class="nav-link" to="/about">About</router-link>
  </nav>
  <!-- Modals (initially hidden) -->
  <CreateStatementModal />
  <router-view />
</template>

<script lang="ts">
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'App',
  components: {
    CreateStatementModal
  }
})
</script>

<style lang="scss">
html, body, #app {
  padding: 0;
  margin: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden; // Prevent scrolling due to rotation
  position: relative; // For pseudo-element positioning
  // background: linear-gradient(144deg, transparent 42%, #1080ca5a, transparent 72%);
}

body::before {
  content: '';
  position: absolute;
  top: -50%; right: -50%; // Expand the element beyond the viewport
  bottom: -50%; left: -50%;
  background: linear-gradient(0deg, transparent 42%, #06394836, transparent 58%);
  z-index: -1; // Place it behind the content

  // Animation
  animation: rotateGradient 120s linear infinite; // Adjust time for speed
}

@keyframes rotateGradient {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #063948; // rgb(6,57,72);
}

nav {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 10vh;
  background: linear-gradient(0deg, transparent 51%, #063948);

  .nav-link {
    font-weight: bold;
    color: #2c3e50;
    margin: 0 1em;    
    transition: 0.3s ease-in-out;

    &.router-link-exact-active {
      color: #42b983;
    }

    &:hover {
      color: #fcb900;
      padding-bottom: 3px;
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
      opacity: 60%;
    }
  }
}
</style>
