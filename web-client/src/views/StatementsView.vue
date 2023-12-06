<template>
    <div class="statements">
        <h1>This is the statements page :)</h1>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Cards from "@/components/Statements/Cards.vue";

@Options({
    components: {
        Cards,
    },
})

export default class StatementsView extends Vue { }
</script>
