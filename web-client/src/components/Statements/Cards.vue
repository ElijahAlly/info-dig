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
import { useStatementsStore } from '@/stores/statements';
import { fetchAllStatements } from '@/api-utils/statementsApi';

export default defineComponent({
    name: 'Cards',
    watch: {
        statements(newValue, oldValue) {
            console.log('newValue', newValue);
            console.log('oldValue', oldValue);
            console.log('sorting statements on update');
            this.sortStatements();
        },
        sortColumnsByPublicView(newValue, oldValue) {
            this.sortStatements();
        },
        sortColumnsByOurTeamView(newValue, oldValue) {
            this.sortStatements();
        },
    },
    mounted() {
        // * TODO: Get recommended (recent, In_Question mostly)
        this.loadAllStatements();
    },
    methods: {
        async loadAllStatements() {
            try {
                const statements: StatementType[] = await fetchAllStatements();
                // console.log('fetched statements', statements);
                useStatementsStore().$patch((state) => {
                    // console.log('exsiting state', state);
                    const statementIds = new Set(state.statements.map(s => s.statement_id));
                    statements.forEach((newStatement) => {
                        if (!statementIds.has(newStatement.statement_id)) {
                            state.statements.push(newStatement);
                        }
                    });
                })
                this.sortStatements();

                // console.log('statements updated', useStatementsStore().statements);
            } catch (error) {
                console.error('Failed to load statements:', error);
            }
        },
        sortStatements() {
            if (this.sortColumnsByPublicView) {
                this.distributeByRating(this.statements || [], 'public_rating');
            } else if (this.sortColumnsByOurTeamView) {
                this.distributeByRating(this.statements || [], 'our_rating');
            } else {
                this.evenlyDistributeElements(this.statements || []);
            }
        },
        evenlyDistributeElements(array: StatementType[]) {
            console.log('evenly distribute statements');
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
        distributeByRating(statements: StatementType[], ratingType: keyof StatementType) {
            console.log('distribute by rating:', ratingType); 
            // Group statements by ratingType => public_rating or our_rating
            const groupedByRating: { [rating: string]: StatementType[] } = {};
            statements.forEach((statement) => {
                if (ratingType === 'public_rating' || ratingType === 'our_rating') {
                    const ratingValue = statement[ratingType];
                    if (!groupedByRating[ratingValue]) {
                        groupedByRating[ratingValue] = [];
                    }
                    groupedByRating[ratingValue].push(statement);
                }
            });

            this.$data.firstChunk = groupedByRating['Proven_Truth'];
            this.$data.secondChunk = groupedByRating['In_Question'];
            this.$data.thirdChunk = groupedByRating['Not_True'];
        },
    },
    data() {
        return {
            componentKey: 0,
            firstChunk: [] as StatementType[] | [],
            secondChunk: [] as StatementType[] | [],
            thirdChunk: [] as StatementType[] | [],
        }
    },
    computed: {
        statements() {
            return useStatementsStore().getAllStatementsFromState;
        },
        sortColumnsByPublicView() {
            return useStatementsStore().sortColumnsByPublicView;
        },
        sortColumnsByOurTeamView() {
            return useStatementsStore().sortColumnsByOurTeamView;
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
