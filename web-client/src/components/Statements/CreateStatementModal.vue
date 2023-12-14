<template>
    <div class="modal" v-if="createStatementModal.isVisible" @click.self="closeModal">
        <div class="modal-content">
            <div class="header-cont">
                <h2 class="header">Create New Statement</h2>
                <span class="close" @click="closeModal">&times;</span>
            </div>
            <form @submit.prevent="submitStatement" class="form-cont">
                <div class="form-group">
                    <label for="statementContent" class="input-label">Content:</label>
                    <input id="statementContent" class="input-field" v-model="createStatementModal.content" required />
                </div>
                <div class="form-group">
                    <label for="statementContext" class="input-label">Context:</label>
                    <textarea id="statementContext" v-model="createStatementModal.context" required class="textarea-field"></textarea>
                </div>
                <!-- <div class="links-group">
                    <div class="links-cont">
                        <label for="statementContext" class="input-label">Link 1:</label>
                        <input id="statementContent" class="input-field" v-model="createStatementModal.links" required />
                    </div>
                    <img class="add-link-btn" src="@/assets/icons/icons8-add-50.png" @click="addLinkInput" alt="add-icon"/>
                </div> -->
                <div class="links-group">
                    <div class="links-cont" v-for="(link, index) in createStatementModal.links" :key="index">
                    <label :for="'statementContent' + index" class="input-label">Link {{ index + 1 }}:</label>
                    <div class="link-input-cont">
                        <input :id="'statementContent' + index" class="input-field" v-model="link.value" required />
                        <img v-if="index !== 0" class="remove-link-btn" src="@/assets/icons/icons8-remove-30.png" @click="() => removeLinkInput(index)" alt="remove-input-icon"/> 
                    </div>
                    </div>
                    <img class="add-link-btn" src="@/assets/icons/icons8-add-50.png" @click="addLinkInput" alt="add-icon"/>
                </div>
                <div class="form-group">
                    <label for="userId">UserId &lt;replace with logged-in user_id from state>:</label>
                    <input id="userId" class="input-field" v-model.number="user_id" type="number" required />
                </div>
                <button type="submit" class="submit-btn">Submit</button>
            </form>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { formatContentToSlug } from '@/api-utils/allApi';
import { createStatement } from '@/api-utils/statementsApi';
import { NewStatementType } from '@/interfaces/statements';
import { useModalsStore } from '@/stores/modals';

export default defineComponent({
    name: 'CreateStatementModal',
    mounted() {
        this.createStatementModal = useModalsStore().getCreateStatements;
    },
    methods: {
        closeModal() {
            useModalsStore().hideCreateStatements();
        },
        openModal() {
            useModalsStore().showCreateStatements();
        },
        async submitStatement() {
            try {
                const statementData: NewStatementType = {
                    content: this.createStatementModal.content,
                    slug: formatContentToSlug(this.createStatementModal.content),
                    context: this.createStatementModal.context,
                    user_id: this.user_id, // * TODO: Replace with logged in user id
                };
                await createStatement(statementData);
                this.closeModal();
            } catch (error) {
                console.error('Failed to create statement:', error);
            }
        },
        addLinkInput() {
            useModalsStore().addLink();
        },
        removeLinkInput(index: number) {
            useModalsStore().removeLink(index);
        }
    },
    data() {
        return {
            createStatementModal: {
                isVisible: false,
                content: '',
                context: '',
                links: [{ value: '' }]
            },
            user_id: 1,
        }
    },
});
</script>

<style scoped>
.modal {
    position: fixed;
    z-index: 1000; /* Increased z-index for higher stack order */
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: rgba(15, 127, 201, 0.5); /* Darker background color */
    backdrop-filter: blur(5px); /* Increased blur effect */
    display: flex;
    align-items: center; /* Center vertically */
    justify-content: center; /* Center horizontally */

    .modal-content {
        background-color: #fff;
        margin: auto; /* Removed specific margin */
        padding: 2rem;
        border: 1px solid #ccc; /* Lighter border */
        width: 50%; /* Adjusted width */
        max-width: 600px; /* Max width for larger screens */
        border-radius: 10px; /* Rounded corners */
        box-shadow: 0 2px 10px rgba(0,0,0,0.2); /* Added shadow */
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
                color: #0a5c8a;
                margin-right: -19px;
            }

            .close {
                color: #333; /* Darker color for close button */
                float: right;
                font-size: 28px;
                font-weight: bold;
                cursor: pointer;
                align-self: flex-start;
            }
            
            .close:hover,
            .close:focus {
                color: #555; /* Subtle hover effect */
                text-decoration: none;
            }
        }

        .form-cont {

            .form-group {
                display: flex;
                flex-direction: column;
                margin-bottom: 1rem;
               
                .input-label {

                }

                .input-field {
                    width: 100%;
                    padding: 0.5rem;
                    margin-top: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 5px;
                }
                
                .textarea-field {
                    width: 100%;
                    max-width: 100%;
                    min-width: 100%;
                    max-height: 240px;
                    padding: 0.5rem;
                    margin-top: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 5px;
                }
            }

            .links-group {
                .links-cont {
                
                    .inputlabel {
                        /* justify-self: flex-start; */
                    }

                    .link-input-cont {
                        display: flex;
                        align-items: center;
                        .input-field {
                            width: 100%;
                            padding: 0.5rem;
                            margin-top: 0.5rem;
                            border: 1px solid #ccc;
                            border-radius: 5px;
                        }

                        .remove-link-btn {
                            cursor: pointer;

                            &:hover {
                                rotate: -21deg 21 3 1;;
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
                background-color: #107fca; /* Primary color for submit button */
                color: white;
                padding: 0.5rem 1rem;
                border: none;
                border-radius: 5px; /* Rounded corners for button */
                cursor: pointer;
                font-weight: bold;
                margin-top: 1rem; /* Space above the button */
            }
            
            .submit-btn:hover {
                background-color: #0a5c8a; /* Darker shade on hover */
            }
        }
    }
}




</style>
