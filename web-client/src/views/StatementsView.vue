<template>
    <div class="page-cont">
        <div class="page-header">
            <div class="actions">
                <GradientButton 
                    :text="'Create New Statement'"
                    :actionOnClick="openCreateModal"
                />
                <GradientButton 
                    :text="'Sort By Public View'" 
                    :actionOnClick="sortStatementsByPublicView"
                    :colorsInnerInitial="'#17bef1 30%, #f1b81b 60%, #f01a53'"
                    :colorsInnerHover="'#17bef1 21%, #f1b81b 51%, #f01a53'"
                    :colorsWrapper="'transparent 66%, gray'"
                    :initialAngle="'158deg'"
                    :hoverAngle="'99deg'"
                />
                <GradientButton 
                    :text="'Sort By Our Team View'" 
                    :actionOnClick="sortStatementsByOurTeamView"
                    :colorsInnerInitial="'#278eae 30%, #ad8a27 60%, #a92649'"
                    :colorsInnerHover="'#278eae 21%, #ad8a27 51%, #a92649'"
                    :colorsWrapper="'transparent 66%, rgb(46, 46, 46)'"
                    :initialAngle="'158deg'"
                    :hoverAngle="'99deg'"
                />
            </div>
        </div>
        <div class="statements">
            <!--
                * Include a search bar [content, context]
                * Put each statement into cards, 3 separate columns, with 3 different sorting modes the user can choose: [public_rating, our_rating, mixed/random] 
                * by statement rating ['Proven_Truth', 'In_Question', 'Not_True'] 
            -->
            <Cards />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import CreateStatementModal from '@/components/Statements/CreateStatementModal.vue';
import Cards from "@/components/Statements/Cards.vue";
import { useStatementsStore } from '@/stores/statements';
import GradientButton from '@/components/GradientButton.vue';
import { useModalsStore } from '@/stores/modals';

export default defineComponent({
    name: 'StatementsView',
    methods: {
        openCreateModal() {
            useModalsStore().showCreateStatements();
        },
        sortStatementsByPublicView() {
            if (useStatementsStore().sortColumnsByPublicView) {
                console.log('make sortColumnsByPublicView false');
                useStatementsStore().$patch({
                    sortColumnsByPublicView: false,
                }) 
            } else {
                console.log('make sortColumnsByPublicView true');
                useStatementsStore().$patch({
                    sortColumnsByOurTeamView: false,
                    sortColumnsByPublicView: true,
                }) 
            }
        },
        sortStatementsByOurTeamView() {
            if (useStatementsStore().sortColumnsByOurTeamView) {
                useStatementsStore().$patch({
                    sortColumnsByOurTeamView: false,
                }) 
            } else {
                useStatementsStore().$patch({
                    sortColumnsByPublicView: false,
                    sortColumnsByOurTeamView: true,
                }) 
            }
        }
    },
    components: {
        Cards,
        CreateStatementModal,
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
    
    .statements {
        height: fit-content;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
}
</style>
