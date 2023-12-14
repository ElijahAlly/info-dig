<template>
    <div class="page-cont">
        <div class="page-header">
            <!-- <h1 class="page-title">This is the statements page :)</h1> -->
            <CreateStatementButton />
        </div>
        <div v-if="statements" class="statements">
            <!--
                * Include a search bar [content, context]
                * Put each statement into cards, 3 separate columns, with 3 different sorting modes the user can choose: [public_rating, our_rating, mixed/random] 
                * by statement rating ['Proven_Truth', 'In_Question', 'Not_True'] 
            -->
            <Cards :cards="statements"/>
        </div>
        <div v-else>
            <p>Loading statements...</p>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchAllStatements } from '@/api-utils/statementsApi'
import { StatementType } from '@/interfaces/statements';
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';
import CreateStatementButton from '@/components/Statements/CreateStatementButton.vue';
import Cards from "@/components/Statements/Cards.vue";
import { mapState } from 'pinia';
import { useStatementsStore } from '@/stores/statements';

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
                const statements = await fetchAllStatements();
                this.statements = statements;
                useStatementsStore().$patch((state) => {
                    state.statements.push(...statements);
                })
            } catch (error) {
                console.error('Failed to load statements:', error);
            }
        },
        openCreateModal() {
            const modal = this.$refs.createModal as any;
            modal.openModal();
        }
    },
    computed: {
        ...mapState(useStatementsStore, ['statements'])
    },
    // data() {
    //     return {
    //         statements: null as StatementType[] | null,
    //     };
    // },
    components: {
        Cards,
        CreateStatementModal,
        CreateStatementButton
    },
});
</script>

<style>
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
        justify-content: center;
        
        .page-title {
            margin: 0;
            padding: 0;
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
