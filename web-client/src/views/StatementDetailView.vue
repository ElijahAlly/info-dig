<template>
    <div class="statement-detail">
        <h1>Statement Detail</h1>
        <div>
            <button @click="openCreateModal">Create New Statement</button>
            <CreateStatementModal ref="createModal" />
        </div>
        <div v-if="statement">
            <p>Statement ID: {{ statement.statement_id }}</p>
            <p>User ID: {{ statement.user_id }}</p>
            <p>Content: {{ statement.content }}</p>
            <p>Context: {{ statement.context }}</p>
            <p>Public Rating: {{ statement.public_rating }}</p>
            <p>Our Rating: {{ statement.our_rating }}</p>
            <p>Created At: {{ statement.created_at }}</p>
            <p>Updated At: {{ statement.updated_at }}</p>
        </div>
        <div v-else>
            <p>Loading statement details...</p>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchAllStatements, fetchStatementById } from '@/api-utils/statementsApi'
import { StatementType } from '@/interfaces/statements';
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';

export default defineComponent({
    name: 'StatementDetailView',
    mounted() {
        const statementSlug = Array.isArray(this.$route.params.slug)
        ? this.$route.params.slug[0]
        : this.$route.params.slug;
        // Use statementId to perform tasks like fetching data
        this.fetchStatementDetails(statementSlug);
    },
    methods: {
        // For fetching all statements
        async loadAllStatements() {
            try {
                const statements = await fetchAllStatements();
                // Process the statements as needed
            } catch (error) {
                console.error('Failed to load statements:', error);
            }
        },
        // For fetching a statement by slug
        async fetchStatementDetails(slug: string) {
            console.log(`Fetching details for statement slug: ${slug}`);
            try {
                const statement = await fetchStatementById(slug);
                console.log(`fetched statement: ${statement}`);
                this.statement = statement;
            } catch (error) {
                console.error(`Failed to load statement with slug ${slug}:`, error);
            }
        },
        openCreateModal() {
            const modal = this.$refs.createModal as any;
            modal.openModal();
        }
    },
    data() {
        return {
            statement: null as StatementType | null, // This will hold the fetched statement data
        };
    },
    components: {
        CreateStatementModal
    },
    watch: {
        '$route'(to, from) {
            if (to.params.id !== from.params.id) {
                this.fetchStatementDetails(to.params.id);
            }
        }
    },
});
</script>
