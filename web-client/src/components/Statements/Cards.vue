<template>
    <div class="cards-cont">
        <!-- * Separate into distinct card components with .map(statement => <Card :statement={...} />) -->
        <!-- * Add Color to statement in the header based on rating & add colored "labels" for both public_rating/our_rating -->
        <div v-if="firstChunk" class="cards-column-1">
            <Card v-for="(statement, i) in firstChunk" :key="i" :statement="statement" />
        </div>
        <p v-if="!firstChunk">
            Loading cards...
        </p>
        <div v-if="secondChunk" class="cards-column-2">
            <Card v-for="(statement, i) in secondChunk" :key="i" :statement="statement" />
        </div>
        <p v-if="!secondChunk">
            Loading cards...
        </p>
        <div v-if="thirdChunk" class="cards-column-3">
            <Card v-for="(statement, i) in thirdChunk" :key="i" :statement="statement" />
        </div>
        <p v-if="!thirdChunk">
            Loading cards...
        </p>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { StatementType } from '@/interfaces/statements';
import Card from '@/components/Statements/Card.vue';

export default defineComponent({
    name: 'Cards',
    mounted() {
        this.evenlyDistributeElements(this.$props.cards || []);
    },
    methods: {
        evenlyDistributeElements(array: StatementType[]) {
            const length = array.length;
            const chunkSize = Math.floor(length / 3);

            // Adjust the first two chunk sizes to distribute elements as evenly as possible
            const remainder = length % 3;
            const firstChunkSize = chunkSize + (remainder > 0 ? 1 : 0);
            const secondChunkSize = chunkSize + (remainder > 1 ? 1 : 0);

            // Split the array into three parts
            this.$data.firstChunk = array.slice(0, firstChunkSize);
            this.$data.secondChunk = array.slice(firstChunkSize, firstChunkSize + secondChunkSize);
            this.$data.thirdChunk = array.slice(firstChunkSize + secondChunkSize);
        }
    },
    data() {
        return {
            firstChunk: null as StatementType[] | null,
            secondChunk: null as StatementType[] | null,
            thirdChunk: null as StatementType[] | null,
        }
    },
    components: {
        Card
    },
    props: {
        cards: {
            type: Array as () => StatementType[],
            default: () => []
        }
    },
})
</script>

<!--
    * Add "scoped" attribute to limit CSS to this component only
    * // Todo: Implement scroll snapping -> https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_scroll_snap/Basic_concepts 
-->

<style scoped lang="scss">
.cards-cont {
    display: flex;
    justify-content: space-evenly; 
    margin-top: 3vh;
    overflow-y: scroll;
    max-height: 81vh;
    width: 90vw;

    .cards-column-1 {
    }
    
    .cards-column-2 {
    }
    
    .cards-column-3 {
    }

    @media (max-width: 700px) {
        .cards-column-3 {
            display: none;
        }
    }
}
</style>
