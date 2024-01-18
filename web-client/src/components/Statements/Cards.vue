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
import { defineComponent, watch } from 'vue';
import { mapState } from 'pinia';
import { StatementType } from '@/interfaces/statements';
import Card from '@/components/Statements/Card.vue';
import { useStatementsStore } from '@/stores/statements';

export default defineComponent({
    name: 'Cards',
    mounted() {
        this.sortStatements();
    },
    methods: {
        sortStatements() {
            if (useStatementsStore().orderColumnsByPublicView) {
                this.distributeByRating(this.statements || []);
            } else {
                this.evenlyDistributeElements(this.statements || []);
            }
        },
        evenlyDistributeElements(array: StatementType[]) {
            console.log('evenly distribute:', array);
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
        },
        distributeByRating(statements: StatementType[]) {
            // Group statements by public_rating
            const groupedByRating: { [rating: string]: StatementType[] } = {};
            statements.forEach((statement) => {
                if (!groupedByRating[statement.public_rating]) {
                    groupedByRating[statement.public_rating] = [];
                }
                groupedByRating[statement.public_rating].push(statement);
            });

            this.$data.firstChunk = groupedByRating['Proven_Truth'];
            this.$data.secondChunk = groupedByRating['In_Question'];
            this.$data.thirdChunk = groupedByRating['Not_True'];
        },
    },
    watch: { // ? does this actually do anything
        statements(newStatements) {
            this.evenlyDistributeElements(newStatements || []);
        },
        orderColumnsByPublicView() {
            this.sortStatements();
        }
    },
    data() {
        return {
            firstChunk: [] as StatementType[] | [],
            secondChunk: [] as StatementType[] | [],
            thirdChunk: [] as StatementType[] | [],
        }
    },
    computed: {
        statements() {
            return useStatementsStore().getAllStatementsFromState;
        },
        orderColumnsByPublicView() {
            return useStatementsStore().orderColumnsByPublicView;
        }
    },
    components: {
        Card
    },
    // props: {
    //     statements: {
    //         type: Array as () => StatementType[],
    //         default: () => []
    //     }
    // },
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
    overflow: hidden;
    max-height: 81vh;
    width: 90vw;

    .cards-column-1 {
        overflow-y: scroll;
        overflow-x: hidden;
        // box-shadow: 0px 9px 3px -4px inset #063948;
        padding: 0 3px 99px 3px;
        border-radius: 6px;

        &::-webkit-scrollbar {
            display: none;
        }
    }
    
    .cards-column-2 {
        overflow-y: scroll;
        overflow-x: hidden;
        // box-shadow: 0px 9px 3px -4px inset #063948;
        padding: 0 3px 99px 3px;
        border-radius: 6px;

        &::-webkit-scrollbar {
            display: none;
        }
    }
    
    .cards-column-3 {
        overflow-y: scroll;
        overflow-x: hidden;
        // box-shadow: 0px 9px 3px -4px inset #063948;
        padding: 0 3px 99px 3px;
        border-radius: 6px;

        &::-webkit-scrollbar {
            display: none;
        }
    }

    @media (max-width: 700px) {
        .cards-column-3 {
            display: none;
        }
    }
}
</style>
