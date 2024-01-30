<template>
    <div class="modal" v-if="editStatementModal.isVisible" @click.self="closeModal">
        <div class="modal-content">
            <div class="header-cont">
                <h2 class="header">Edit Statement</h2>
                <span class="close" @click="closeModal">&times;</span>
            </div>
            <form @submit.prevent="submitStatement" class="form-cont">
                <div class="form-group">
                    <label for="statementContent" class="input-label">Content:</label>
                    <input id="statementContent" class="input-field" v-model="editStatementModal.content" required />
                </div>
                <div class="form-group">
                    <label for="statementContext" class="input-label">Context:</label>
                    <textarea
                        id="statementContext"
                        v-model="editStatementModal.context"
                        class="textarea-field"
                        required
                    ></textarea>
                </div>
                <div class="links-group">
                    <div class="links-cont" v-for="(link, index) in editStatementModal.links" :key="index">
                        <label :for="'statementLinkUrl' + index" class="input-label">Link {{ index + 1 }}:</label>
                        <div class="link-input-cont">
                            <input :id="'statementLinkUrl' + index" class="input-field link-field" v-model="link.url"
                                placeholder="URL" />
                            <input :id="'statementLinkName' + index" class="input-field link-field" v-model="link.name"
                                placeholder="Short Name Alternative" />
                            <img v-if="index !== 0" class="remove-link-btn" src="@/assets/icons/icons8-remove-30.png"
                                @click="() => removeLinkInput(index)" alt="remove-input-icon" />
                        </div>
                    </div>
                    <img class="add-link-btn" src="@/assets/icons/icons8-add-50.png" @click="addLinkInput" alt="add-icon" />
                </div>
                <button type="submit" class="submit-btn">Update</button>
            </form>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { fetchStatementById, updateStatement } from '@/api-utils/statementsApi';
import { StatementType, UpdateStatementType } from '@/interfaces/statements';
import { useModalsStore } from '@/stores/modals';
import { useStatementsStore } from '@/stores/statements';
import { createReadableSlug, containsValidLinks } from '@/utils/statementsHelpers';

export default defineComponent({
    name: 'EditStatementModal',
    mounted() {
        this.editStatementModal = useModalsStore().editStatement;
    },
    methods: {
        closeModal() {
            useModalsStore().hideEditStatements();
        },
        openModal() {
            useModalsStore().showCreateStatements();
        },
        async submitStatement() {
            try {
                const statementData: UpdateStatementType = {
                    statement_id: this.editStatementModal.statement_id,
                    content: this.editStatementModal.content,
                    // only create new slug if content is not the same as when first loaded
                    slug: useStatementsStore().selectedStatement.content === this.editStatementModal.content 
                        ? this.editStatementModal.content
                        : createReadableSlug(this.editStatementModal.content), 
                    context: this.editStatementModal.context,
                    links: containsValidLinks(this.editStatementModal.links)
                };
                const newStatement = await updateStatement(statementData);
                if (newStatement) {
                    useStatementsStore().$patch({
                        selectedStatement: {
                            ...useStatementsStore().selectedStatement,
                            ...newStatement 
                        } 
                    });
                }
                this.closeModal();
            } catch (error) {
                console.error('Failed to create statement:', error);
            }
        },
        addLinkInput() {
            useModalsStore().addEditLink();
        },
        removeLinkInput(index: number) {
            useModalsStore().removeEditLink(index);
        }
    },
    props: {
        statement: {
            type: Object as () => StatementType,
            default: () => ({})
        }
    },
    data() {
        return {
            editStatementModal: {
                isVisible: false,
                content: '',
                statement_id: 0,
                context: '',
                links: [{ url: '', name: '' }]
            },
        }
    },
});
</script>

<style scoped>
.modal {
    position: fixed;
    z-index: 1000;
    /* Increased z-index for higher stack order */
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: #0639484d;
    backdrop-filter: blur(5px);
    display: flex;
    align-items: center;
    justify-content: center;

    .modal-content {
        background-color: aliceblue;
        margin: auto;
        padding: 2rem;
        border: 1px solid #ccc;
        width: 50%;
        max-width: 600px;
        border-radius: 10px;
        box-shadow: 0 2px 10px #0639484d;
        display: flex;
        flex-direction: column;
        align-items: center;

        .header-cont {
            width: 100%;
            display: flex;
            justify-content: center;
            align-items: center;

            .header {
                width: 100%;
                color: #063948;
                margin-right: -19px;
            }

            .close {
                color: #06394874;
                float: right;
                font-size: 28px;
                font-weight: bold;
                cursor: pointer;
                align-self: flex-start;
                clip-path: circle();
                padding: 3px;
            }

            .close:hover,
            .close:focus {
                color: #063948;
                text-decoration: none;
            }
        }

        .form-cont {
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            height: 100%;

            .form-group {
                display: flex;
                flex-direction: column;
                align-items: center;
                margin-bottom: 1rem;
                width: 100%;
                height: 100%;

                /* .input-label {

                } */

                .input-field {
                    width: 60%;
                    padding: 0.5rem;
                    margin-top: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 5px;
                }

                .textarea-field {
                    min-width: 60%;
                    width: 60%;
                    max-width: 100%;
                    max-height: 240px;
                    padding: 0.5rem;
                    margin-top: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 5px;
                }
            }

            .links-group {
                display: flex;
                flex-direction: column;
                align-items: center;
                width: 100%;
                margin-bottom: 1rem;

                .links-cont {
                    width: 60%;

                    .inputlabel {
                        /* justify-self: flex-start; */
                    }

                    .link-input-cont {
                        width: 100%;
                        display: flex;
                        align-items: center;
                        justify-content: space-between;

                        .input-field {
                            width: 45%;
                            padding: 0.5rem;
                            margin-top: 0.5rem;
                            margin-right: 0.5rem;
                            border: 1px solid #ccc;
                            border-radius: 5px;
                        }

                        .remove-link-btn {
                            cursor: pointer;

                            &:hover {
                                rotate: -21deg 21 3 1;
                                ;
                            }
                        }
                    }
                }

                .add-link-btn {
                    height: 36px;
                    margin-top: 12px;
                    cursor: pointer;
                    clip-path: circle();
                }
            }

            .submit-btn {
                background-color: #107fca;
                /* Primary color for submit button */
                color: aliceblue;
                padding: 0.5rem 1rem;
                border: none;
                border-radius: 5px;
                /* Rounded corners for button */
                cursor: pointer;
                font-weight: bold;
                margin-top: 1rem;
                /* Space above the button */
            }

            .submit-btn:hover {
                background-color: #0a5c8a;
                /* Darker shade on hover */
            }
        }
    }
}</style>
