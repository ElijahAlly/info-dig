<template>
    <h1>This is the statements page :)</h1>
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
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchAllStatements } from '@/api-utils/statementsApi'
import { StatementType } from '@/interfaces/statements';
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';
import Cards from "@/components/Statements/Cards.vue";

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
            } catch (error) {
                console.error('Failed to load statements:', error);
            }
        },
        openCreateModal() {
            const modal = this.$refs.createModal as any;
            modal.openModal();
        }
    },
    data() {
        return {
            statements: null as StatementType[] | null,
        };
    },
    components: {
        Cards,
        CreateStatementModal
    },
});
</script>

