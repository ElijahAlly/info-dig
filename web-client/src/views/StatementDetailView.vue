<template>
    <div class="statement-detail">
        <div v-if="statement" class="header">
            <div class="copy-btn-cont">
                <img 
                src="@/assets/icons/icons8-copy-16.png"
                class="copy-btn"
                @click="copyTitleToClipboard"
                >
            </div>
            <h1 class="title">
                {{ statement.content }}
            </h1>
        </div>
        <div class="actions">
            <CreateStatementButton />
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
import CreateStatementButton from '@/components/Statements/CreateStatementButton.vue';

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
        copyTitleToClipboard() {
            navigator.clipboard.writeText(this.statement?.content || '<Could not copy statement title>');
        }
    },
    data() {
        return {
            statement: null as StatementType | null, // This will hold the fetched statement data
        };
    },
    components: {
        CreateStatementButton
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

<style scoped lang="scss">
.statement-detail {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 0;
    padding: 0;

    .header {
        display: flex;
        align-items: center;
        justify-content: center;
        
        .copy-btn-cont {
            display: flex;
            align-items: center;
            margin: 0 15px;

            .copy-btn {
                height: 16px;
                cursor: pointer;
                
                &:hover {
                    height: 20px;
                }
            }
        }

        .copy-btn-cont:hover {
            margin-left: 13px;
            margin-right: 13px;
        }

        .title {
            max-width: 51%;
        }
    }

    .actions {
        display: flex;
        justify-content: center;
        width: fit-content;
    }
}
</style>
