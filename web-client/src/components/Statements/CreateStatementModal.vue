<template>
    <div class="modal" v-if="isVisible">
        <div class="modal-content">
            <span class="close" @click="closeModal">&times;</span>
            <h2>Create New Statement</h2>
            <form @submit.prevent="submitStatement">
                <div class="form-group">
                    <label for="statementContent">Content:</label>
                    <input id="statementContent" v-model="content" required />
                </div>
                <div class="form-group">
                    <label for="statementContext">Context:</label>
                    <textarea id="statementContext" v-model="context" required></textarea>
                </div>
                <div class="form-group">
                    <label for="userId">User Id:</label>
                    <input id="userId" v-model.number="user_id" type="number" required />
                </div>
                <button type="submit">Submit</button>
            </form>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { formatContentToSlug } from '@/api-utils/allApi';
import { createStatement } from '@/api-utils/statementsApi';
import { NewStatementType } from '@/interfaces/statements';

export default defineComponent({
    name: 'CreateStatementModal',
    methods: {
        closeModal() {
            this.isVisible = false;
        },
        openModal() {
            this.isVisible = true;
        },
        async submitStatement() {
            const statementData: NewStatementType = {
                content: this.content,
                slug: formatContentToSlug(this.content),
                context: this.context,
                user_id: this.user_id,
            };
            console.log(`Creating statement:`, statementData);
            try {
                const newStatement = await createStatement(statementData);
                console.log('New statement created:', newStatement);
                this.closeModal();
            } catch (error) {
                console.error('Failed to create statement:', error);
            }
        }
    },
    data() {
        return {
            isVisible: false,
            content: '',
            context: '',
            user_id: 0,
        }
    },
});
</script>

<style scoped>
.modal {
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: rgba(0, 0, 0, 0.4);
}

.modal-content {
    background-color: #fefefe;
    margin: 15% auto;
    padding: 20px;
    border: 1px solid #888;
    width: 80%;
}

.close {
    color: #aaa;
    float: right;
    font-size: 28px;
    font-weight: bold;
}

.close:hover,
.close:focus {
    color: black;
    text-decoration: none;
    cursor: pointer;
}</style>
