<template>
    <div class="page-cont">
        <div class="page-header">
            <div class="actions">
                <CreateStatementButton />
                <div class="order-by-public-view-wrapper">
                    <button @click="toggleOrderByPublicView" class="order-by-public-view">
                        Order By Public View
                    </button>
                </div>
            </div>
        </div>
        <div class="statements">
            <!--
                * Include a search bar [content, context]
                * Put each statement into cards, 3 separate columns, with 3 different sorting modes the user can choose: [public_rating, our_rating, mixed/random] 
                * by statement rating ['Proven_Truth', 'In_Question', 'Not_True'] 
            -->
            <Cards />
        </div>
        <!-- <div v-else>
            <p>Loading statements...</p>
        </div> -->
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchAllStatements } from '@/api-utils/statementsApi'
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';
import CreateStatementButton from '@/components/Statements/CreateStatementButton.vue';
import Cards from "@/components/Statements/Cards.vue";
import { mapState } from 'pinia';
import { useStatementsStore } from '@/stores/statements';
import { StatementType } from '@/interfaces/statements';

export default defineComponent({
    name: 'StatementDetailView',
    mounted() {
        // * TODO: Get recommended (recent, In_Question mostly) 
        this.loadAllStatements();
    },
    methods: {
        // For fetching all statements
        async loadAllStatements() {
            try {
                const statements: StatementType[] = await fetchAllStatements();
                console.log('fetched statements', statements);
                useStatementsStore().$patch((state) => {
                    console.log('exsiting state', state);
                    const statementIds = new Set(state.statements.map(s => s.statement_id));
                    statements.forEach((newStatement) => {
                        if (!statementIds.has(newStatement.statement_id)) {
                            state.statements.push(newStatement);
                        }
                    });
                })
                console.log('statements updated', useStatementsStore().statements);
            } catch (error) {
                console.error('Failed to load statements:', error);
            }
        },
        openCreateModal() {
            const modal = this.$refs.createModal as any;
            modal.openModal();
        },
        toggleOrderByPublicView() {
            if (useStatementsStore().orderColumnsByPublicView) {
                useStatementsStore().$patch({
                    orderColumnsByPublicView: false,
                }) 
            } else {
                useStatementsStore().$patch({
                    orderColumnsByPublicView: true,
                }) 
            }
        }
    },
    components: {
        Cards,
        CreateStatementModal,
        CreateStatementButton
    },
});
</script>

<style scoped lang="scss">
.page-cont {
    height: 90vh;
    width: 100vw;
    margin: 0;
    padding: 0;
    position: relative;
    
    .page-header {
        height: 6vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-evenly;
        padding: 20px 0;
        
        .page-title {
            margin: 0;
            padding: 0;
        }

        .actions {
            display: flex;
            justify-content: space-evenly;
            width: 100%;

            .order-by-public-view-wrapper {
                background: linear-gradient(158deg, transparent 66%, gray);
                width: fit-content;
                height: fit-content;
                border-radius: 6px;
                margin: 0;
                padding: 0;
                display: flex;
                align-items: center;
                justify-content: center;
                
                .order-by-public-view {
                    outline: none;
                    text-decoration: none;
                    background: linear-gradient(158deg, #37c4ef 30%, #efbe37 60%, #ef3768);
                    width: fit-content;
                    height: 33px;
                    border-width: 0;
                    border-radius: 6px;
                    color: white;
                    cursor: pointer;
                    font-size: small;
                    font-weight: bold;
                    padding: 6px;
                }
                
                &:hover {
                    rotate: -12deg 21 3 1;
                }
                
                &:hover > .order-by-public-view {
                    background: linear-gradient(99deg, #37c4ef 21%, #efbe37 51%, #ef3768);
                }
            }
        }
    }
    
    .statements {
        height: fit-content;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
}
</style>
