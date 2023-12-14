<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
  </div>
  <!-- * Icons8 link for icon use // TODO: Put in footer -->
  <a target="_blank" href="https://icons8.com/icon/43524/copy">Copy</a> icon by <a target="_blank" href="https://icons8.com">Icons8</a>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    msg: String,
  },
})
export default class HelloWorld extends Vue {
  msg!: string;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
