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
        <div class="card-component">
            <div class="rating-cont">
                <div class="rating-label public-rating">
                    <h5 class="title">Public View</h5>
                    <h5 class="value" :style="{ 'color': Styles.publicRatingColor }">
                        {{ statement.public_rating }}
                    </h5>
                </div>
                <div class="rating-label our-rating">
                    <h5 class="title">Our Team View</h5>
                    <h5 class="value" :style="{ 'color': Styles.ourRatingColor }" >
                        {{ statement.our_rating }} 
                    </h5>
                </div>
            </div>
            <div class="hero-cont" :style="{ 'background-color': Styles.publicRatingColor }" @click="navigateToDetail">
                <h3 class="title" :title="statement.content" :style="{ 'text-decoration-style': Styles.decorationStyle }">
                    {{ formatTitle(statement.content) }}
                </h3>
            </div>
            <div class="card-body">
                <p>{{ formatContext(statement.context) }}</p>
            </div>
            <!-- <Links v-for="(statement, i) in firstChunk" :key="i" :statement="statement" /> -->
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
                publicRatingColor: 'gray',
                ourRatingColor: 'gray',
                secondaryColor: '#063948',
                decorationStyle: 'solid'
            };

            const getCorrectColor = (rating: string) => {
                switch (rating) {
                    case 'Proven_Truth':
                        return 'rgb(55, 196, 239)'; 
                    case 'In_Question':
                        return 'rgb(239, 190, 55)';
                    case 'Not_True':
                        return 'rgb(239, 55, 104)';
                    default:
                        console.log('rating', rating);
                        return 'gray';
                }
            }

            styles.publicRatingColor = getCorrectColor(this.$props.statement.public_rating);
            styles.ourRatingColor = getCorrectColor(this.$props.statement.our_rating);
            return styles;
        },
    },
    methods: {
        formatTitle(title: string) {
            const maxTitleLength = 81;
            if (title.length < maxTitleLength) return title;
            return title.slice(0, maxTitleLength) + '...';
        },
        // Keep as separate funcs because in future, context will need additional formatting
        formatContext(context: string) {
            const maxTitleLength = 150;
            if (context.length < maxTitleLength) return context;
            return context.slice(0, maxTitleLength) + '...';
        },
        navigateToDetail() {
            router.push({ name: 'statement-detail', params: { slug: this.statement.slug } });
        }
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
        max-height: 420px;
        margin-bottom: 24px;
        display: flex;
        flex-direction: column;
        background-color: white;
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
                color: white;
                
                .title {
                    font-size: medium;
                    font-weight: 500;
                    margin: 0;
                    padding: 0;
                }
                
                .value {
                    margin: 0;
                    padding: 0;
                    font-size: small;
                    font-weight: 400;
                    text-decoration-line: overline;
                    text-decoration-style: wavy;
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
            min-height: 72px;
            max-height: 200px;
            width: 24fw;
            padding: 15px;
            background-color: #107fca;
            transition: 0.3s ease-in-out;

            .title {
                cursor: pointer;
                text-decoration-style: dotted;
                font-size: medium;
                font-weight: 500;
                
                &:hover {
                    text-decoration-line: underline;
                }
            }
        }

        .card-body {
            padding: 9px;
        }
    }
}
</style>
