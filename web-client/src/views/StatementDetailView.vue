<template>
    <div v-if="(selectedStatement as StatementType)" class="statement-detail">
        <div class="header">
            <h1 class="title">
                {{ selectedStatement.content }}
            </h1>
            <div class="actions">
                <span
                    id="copy-btn"
                    @click="copyTitleToClipboard"
                >Copy Title</span>
                <span
                    id="edit-btn"
                    @click="editStatementModal"
                >Edit</span>
            </div>
        </div>
        <div class="rating-cont">
            <div class="rating public">
                <p class="title">Public Rating</p>
                <p 
                    class="value"
                    :style="{ backgroundColor: Styles.publicRating.backgroundColor, color: Styles.publicRating.color }"
                >{{ formatRating(selectedStatement.public_rating) }}</p>
            </div>
            <div class="your-vote">
                <p>Add Your Vote</p>
            </div>
            <div class="rating our-team">
                <p class="title">Our Rating</p>
                <p 
                    class="value"
                    :style="{ backgroundColor: Styles.ourTeamRating.backgroundColor, color: Styles.ourTeamRating.color }"
                >{{ formatRating(selectedStatement.our_rating) }}</p>
            </div>
        </div>
        <div class="context">
            <!-- <p>Statement ID: {{ statement.statement_id }}</p> -->
            <!-- <p>User ID: {{ statement.user_id }}</p> -->
            <p>Context: {{ selectedStatement.context }}</p>
            <!-- <p>Created At: {{ statement.created_at }}</p>
            <p>Updated At: {{ statement.updated_at }}</p> -->
        </div>
    </div>
    <div v-else>
        <p>Loading statement details...</p>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchStatementById } from '@/api-utils/statementsApi'
import { getCorrectColor, formatRating } from '@/utils/statementsHelpers';
import { useModalsStore } from '@/stores/modals';
import EditStatementModal from '@/components/Statements/Card.vue';
import { useStatementsStore } from '@/stores/statements';
import { StatementType } from '@/interfaces/statements';

export default defineComponent({
    name: 'StatementDetailView',
    mounted() {
        const statementSlug = Array.isArray(this.$route.params.slug)
        ? this.$route.params.slug[0]
        : this.$route.params.slug;
        this.fetchStatementDetails(statementSlug);
    },
    methods: {
        // For fetching a statement by slug
        async fetchStatementDetails(slug: string) {
            try {
                if (!slug) return null;
                const statement = await fetchStatementById(slug);
                useStatementsStore().$patch({ selectedStatement: statement });
                this.selectedStatement = statement;
            } catch (error) {
                console.error(`Failed to load statement with slug ${slug}:`, error);
            }
        },
        async copyTitleToClipboard() {
            navigator.clipboard.writeText(this.selectedStatement?.content || '<Could not copy statement title>');
            const text = await navigator.clipboard.readText();
            if (text !== '' && text !== '<Could not copy statement title>') {
                const copyBtn = document.getElementById('copy-btn')
                if (copyBtn) {
                    copyBtn.innerHTML = 'Copied!';
                    copyBtn.style.backgroundColor = 'lightgreen';
                    setTimeout(() => {
                        copyBtn.innerHTML = 'Copy Title';
                        copyBtn.style.backgroundColor = 'aliceblue';
                    }, 1500);
                }
            }
        },
        editStatementModal() {
            if (this.selectedStatement) {
                useModalsStore().showEditStatements(this.selectedStatement);
            }
        },
        formatRating(rating: string) {
            return formatRating(rating || '');
        }
    },
    computed: {
        Styles() {
            const styles = {
                publicRating: {
                    backgroundColor: '#063948',
                    color: 'aliceblue'
                },
                ourTeamRating: {
                    backgroundColor: '#063948',
                    color: 'aliceblue'
                }
            };

            styles.publicRating = getCorrectColor(this.selectedStatement.public_rating || '');
            styles.ourTeamRating = getCorrectColor(this.selectedStatement.our_rating || '');
            return styles;
        },
        selectedStatement() {
            return useStatementsStore().selectedStatement;
        }
    },
    watch: {
        selectedStatement(newValue, oldValue) {
            console.log("newstatement", newValue);
            console.log("oldstatement", oldValue);
            this.fetchStatementDetails(newValue.slug || '');
        },
    },
    // data() {
    //     return {
    //         statement: null as StatementType | null, // This will hold the fetched statement data
    //     };
    // },
    components: {
        EditStatementModal,
    },
    // '$route'(to, from) {
    //     if (to.params.id !== from.params.id) {
    //         this.fetchStatementDetails(to.params.id);
    //     }
    // },
});
</script>

<style scoped lang="scss">
.statement-detail {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 0;
    padding: 0;
    background-color: #063948;
    color: aliceblue;

    .header {
        width: 100%;
        height: 100%;
        min-height: 12vh;
        max-height: 18vh;
        display: flex;
        align-items: center;
        justify-content: space-evenly;
        padding: 0 10%;
        
        .actions {
            width: 9%;
            height: 12vh;
            display: flex;
            flex-direction: column;
            // justify-content: space-between;
            padding: 12px 0;

            #copy-btn {
                background-color: aliceblue;
                color: #063948;
                padding: 6px;
                cursor: pointer;
                border-radius: 6px;
                margin-bottom: 12px;

                &:hover {
                    background-color: aqua;
                }
            }

            #edit-btn {
                background-color: transparent;
                color: aliceblue;
                padding: 6px;
                cursor: pointer;
                border-radius: 6px;
                border: 1px solid aliceblue;

                &:hover {
                    color: aqua;
                    border-color: aqua;
                }
            }
        }

        .title {
            height: 100%;
            max-width: 71%;
            color: aliceblue;
        }
    }

    .rating-cont {
        background-color: #063948;
        width: 100%;
        height: 90px;
        color: #063948;
        display: flex;

        .rating {
            width: 40%;
            height: 100%;

            .title {
                background-color: aliceblue;
                display: flex;
                align-items: center;
                justify-content: center;
                height: 50%;
                margin: 0;
            }

            .value {
                display: flex;
                align-items: center;
                justify-content: center;
                height: 50%;
                margin: 0;
            }
        }

        .public {

            .title {
                border-top-right-radius: 9px;
            }

            .value {
                
            }
        }

        .your-vote {
            background-color: #063948;
            color: aliceblue;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            width: 20%;
            height: 100%;
        }

        .our-team {

            .title {
                border-top-left-radius: 9px;
            }

            .value {
                
            }
        }
    }

    .context {
        width: 100%;
        min-height: 42vh;
        background-color: aliceblue;
        color: #063948; 
    }
}
</style>
