<template>
    <div class="create-btn-wrapper">
        <button @click="openCreateModal" class="create-statement-btn">Create New Statement</button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useModalsStore } from '@/stores/modals';

export default defineComponent({
    name: 'CreateStatementButton',
    methods: {
        openCreateModal() {
            useModalsStore().showCreateStatements();
            console.log('open from btn view: createStatementModal', useModalsStore().getCreateStatements.isVisible);
        }
    },
})
</script>

<style scoped lang="scss">
.create-btn-wrapper {
    background: linear-gradient(158deg, transparent 66%, gray);
    width: fit-content;
    height: fit-content;
    border-radius: 6px;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    
    .create-statement-btn {
        outline: none;
        text-decoration: none;
        background: linear-gradient(158deg, #063948 30%, #107fca);
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

    &:hover > .create-statement-btn {
        background: linear-gradient(148deg, #063948 3%, #107fca);
    }
}
</style>