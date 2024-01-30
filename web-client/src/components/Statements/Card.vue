<template>
    <!-- * Separate into distinct card components with .map(statement => <Card :statement={...} />) -->
    <!-- * Add Color to statement in the header based on rating & add colored "labels" for both public_rating/our_rating -->
    <div v-if="statement" class="component-wrapper">
        <!-- Public Rating:
            For "Proven_Truth": "Publicly Verified"
            For "In_Question": "Publicly Debated"
            For "Not_True": "Publicly Disputed"
        Our Rating:
            For "Proven_Truth": "Verified by our team"
            For "In_Question": "Under Our Review"
            For "Not_True": "Discredited by our team" -->
        <div
            class="card-component"
            @mouseover="() => {
                addHoverClass('Public_Rating');
                addHoverClass('Our_Rating');
            }"
            @mouseleave="() => {
                removeHoverClass('Public_Rating');
                removeHoverClass('Our_Rating');
            }"
        >
            <div class="rating-cont">
                <div class="rating-label public-rating">
                    <h5
                        class="title"
                        :style="{ 'text-decoration-color': Styles.publicRatingColor.backgroundColor }"
                        ref="publicRatingElement"
                    >Public View</h5>
                    <h5 class="value" :style="{ 'color': Styles.publicRatingColor.backgroundColor }">
                        {{ statement.public_rating }}
                    </h5>
                </div>
                <div class="rating-label our-rating">
                    <h5
                        class="title"
                        :style="{ 'text-decoration-color': Styles.ourRatingColor.backgroundColor }"
                        ref="ourRatingElement"
                    >Our Team View</h5>
                    <h5 class="value" :style="{ 'color': Styles.ourRatingColor.backgroundColor }">
                        {{ statement.our_rating }} 
                    </h5>
                </div>
            </div>
            <div 
                class="hero-cont" 
                :style="{
                    backgroundColor: Styles.publicRatingColor.backgroundColor,
                    borderBottomRightRadius: statement.links.length ? '0' : '3px',
                    borderBottomLeftRadius: statement.links.length ? '0' : '3px' 
                }"
            >
                <h3 
                    class="hero-title"
                    :title="statement.content"
                    @click="navigateToDetail"
                    :style="{ 'color': Styles.heroTextColor }"
                >
                    {{ formatTitle(statement.content) }}
                </h3>
            </div>
            <div class="card-body">
                <p>{{ formatContext(statement.context) }}</p>
            </div>
            <Links :links="statement.links" :publicRatingColor="Styles.publicRatingColor.backgroundColor" />
        </div>
    </div>
    <p v-else>
        Loading cards...
    </p>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { StatementType } from '@/interfaces/statements';
import router from '@/router';
import Links from './Links.vue';
import { getCorrectColor } from '@/utils/statementsHelpers';
import { useStatementsStore } from '@/stores/statements';

export default defineComponent({
    name: 'Card',
    props: {
        statement: {
            type: Object as () => StatementType,
            default: () => ({})
        }
    },
    computed: {
        Styles() {
            const styles = {
                publicRatingColor: {
                    backgroundColor: '#063948',
                    color: 'aliceblue'
                },
                ourRatingColor: {
                    backgroundColor: '#063948',
                    color: 'aliceblue'
                },
                secondaryColor: '#063948',
                heroTextColor: 'white' // dark blue
            };
            const getHeroTextColor = (rating: string) => {
                switch (rating) {
                    case 'Not_True':
                        return 'aliceblue';
                    default:
                        return styles.heroTextColor;
                }
            };

            styles.publicRatingColor = getCorrectColor(this.$props.statement.public_rating);
            styles.ourRatingColor = getCorrectColor(this.$props.statement.our_rating);
            styles.heroTextColor = getHeroTextColor(this.$props.statement.public_rating);
            return styles;
        },
    },
    methods: {
        formatTitle(title: string) {
            const maxTitleLength = 81;
            if (title.length < maxTitleLength)
                return title;
            return title.slice(0, maxTitleLength) + '...';
        },
        formatContext(context: string) {
            const maxTitleLength = 150;
            if (context.length < maxTitleLength)
                return context;
            return context.slice(0, maxTitleLength) + '...';
        },
        navigateToDetail() {
            useStatementsStore().$patch({ selectedStatement: this.statement });
            router.push({ name: 'statement-detail', params: { slug: this.statement.slug } });
        },
        addHoverClass(rating_type: string) {
            switch (rating_type) {
                case 'Our_Rating':
                    const ourRatingElement = this.$refs.ourRatingElement as HTMLElement;
                    ourRatingElement.classList.add('hovered');
                    return;
                case 'Public_Rating':
                    const publicRatingElement = this.$refs.publicRatingElement as HTMLElement;
                    publicRatingElement.classList.add('hovered');
                    return;
            }
        },
        removeHoverClass(rating_type: string) {
            switch (rating_type) {
                case 'Our_Rating':
                    const ourRatingElement = this.$refs.ourRatingElement as HTMLElement;
                    ourRatingElement.classList.remove('hovered');
                    return;
                case 'Public_Rating':
                    const publicRatingElement = this.$refs.publicRatingElement as HTMLElement;
                    publicRatingElement.classList.remove('hovered');
                    return;
            }
        },
    },
    components: { 
        Links
    }
})
</script>

<!--
    * Add "scoped" attribute to limit CSS to this component only
    * // Todo: Implement scroll snapping -> https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_scroll_snap/Basic_concepts 
-->

<style scoped lang="scss">
.component-wrapper {
    background: linear-gradient(158deg, transparent 66%, #063948);
    border-radius: 6px;

    .card-component {
        position: relative;
        width: 24vw;
        min-width: 180px;
        min-height: 100px;
        height: fit-content;
        margin-bottom: 24px;
        display: flex;
        flex-direction: column;
        background-color: aliceblue;
        border: 2px solid #063948;
        border-radius: 6px;
        transition: 0.15s ease-in-out;

        &:hover {
            rotate: -12deg 21 3 1;
        }

        .rating-cont {
            width: 100%;
            display: flex;
            justify-content: space-between;
            background-color: #063948;

            .rating-label {
                display: flex;
                flex-direction: column;
                height: 42px;
                width: fit-content;
                padding: 6px;
                color: aliceblue;
                
                .title {
                    color: #063948;
                    display: flex;
                    font-size: small;
                    font-weight: 500;
                    margin: 0;
                    padding: 0;
                    text-decoration-line: underline;
                    text-decoration-style: dashed;
                    text-underline-offset: 1px;
                    transition: 0.3s;
                    line-height: 12px;
                }

                .value {
                    margin: 0;
                    margin-top: 3px;
                    padding: 0;
                    font-size: small;
                    font-weight: 400;
                }

                .hovered {
                    color: aliceblue;
                    text-underline-offset: 4px;
                    text-decoration-style: solid;
                }
            }

            .public-rating {
                border-top-left-radius: 4px;
                border-bottom-right-radius: 6px;
            }
    
            .our-rating {
                border-top-right-radius: 4px;
                border-bottom-left-radius: 6px;
            }
        }

        .hero-cont {
            min-height: 60px;
            max-height: 200px;
            width: 24fw;
            padding: 15px;
            transition: 0.3s ease-in-out;

            .hero-title {
                display: flex;
                height: fit-content;
                width: inherit;
                cursor: pointer;
                text-decoration-style: dashed;
                text-underline-offset: 6px;
                line-height: 30px;
                font-size: medium;
                font-weight: 500;
                
                &:hover {
                    text-decoration-line: underline;
                }
            }
        }

        .card-body {
            display: none; // temporary, unhide later
            max-height: 42px;
            padding: 9px;
        }
    }
}
</style>
