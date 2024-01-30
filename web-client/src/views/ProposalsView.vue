<template>
    <div class="page-cont">
        <div class="page-header">
            <div class="actions">
            </div>
        </div>
        <div class="proposals">
            <Slides />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import GradientButton from '@/components/GradientButton.vue';
import Slides from '@/components/Proposals/Slides.vue';

export default defineComponent({
    name: 'ProposalsView',
    // methods: {
        
    // },
    components: {
        Slides,
        GradientButton
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

        .actions {
            display: flex;
            justify-content: space-evenly;
            width: 100%;
        }
    }

    .proposals {
        height: fit-content;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
}
</style>

