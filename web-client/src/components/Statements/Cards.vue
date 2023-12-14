<template>
    <div class="cards-cont">
        <!-- * Separate into distinct card components with .map(statement => <Card :statement={...} />) -->
        <!-- * Add Color to statement in the header based on rating & add colored "labels" for both public_rating/our_rating -->
        <div v-if="firstChunk" class="cards-column-1">
            <p>
                {{ firstChunk }}
            </p>
        </div>
        <div v-if="secondChunk" class="cards-column-2">
            <p>
                {{ secondChunk }}
            </p>
        </div>
        <div v-if="thirdChunk" class="cards-column-3">
            <p>
                {{ thirdChunk }}
            </p>
        </div>
        <div v-else>
            Loading cards...
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { StatementType } from '@/interfaces/statements';

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
    props: {
        cards: [],
    },
})
</script>

<!-- 
    * Add "scoped" attribute to limit CSS to this component only
    * // Todo: Implement scroll snapping -> https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_scroll_snap/Basic_concepts 
-->
<style scoped lang="scss">
.cards-cont {
    padding: 51px;
    display: flex;
    transition: 0.3s ease-in-out;

    .cards-column-1:hover {
        background-color: #107fca;
        cursor: pointer;
    }

    .cards-column-2:hover {
        background-color: #fcb900;
        cursor: pointer;
    }

    .cards-column-3:hover {
        background-color: #eb144c;
        cursor: pointer;
    }
}

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
}</style>
