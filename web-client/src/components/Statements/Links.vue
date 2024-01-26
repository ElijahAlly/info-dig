<template>
    <div v-if="links.length" class="links-wrapper" :style="{ backgroundColor: publicRatingColor }">
        <span class="links-title">Links</span>
        <a
            v-for="({ url, name }, i) in links.slice(0, 7)"
            :key="i"
            class="link-cont"
            :style="{
                borderBottomLeftRadius: '0',
                borderBottomRightRadius: i === 6 ? '4px' : '0',
                borderTopRightRadius: i !== 7 ? '4px' : '0',
                borderTopLeftRadius: '6px', // i !== 0 ? '4px' : '0'
                background: linkStyles.background(i + 1)
            }"
            @mouseover="hover=i+1"
            @mouseleave="hover=0"
            :href="url"
            target="_blank"
            :title="`Link: ${name}`"
        >{{ i + 1 }}</a>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { LinkType } from '@/interfaces/statements';

export default defineComponent({
    name: 'Links',
    mounted() {
        console.log('links', this.links);
    },
    props: {
        links: {
            type: Object as () => LinkType[],
            default: () => ([{ url: "", name: ""}])
        },
        publicRatingColor: {
            type: String,
            required: true
        }
    },
    computed: {
        linkStyles() {
            return ({
                background: (i: number) => i === this.hover
                    ? 'rgba(255, 255, 255, 0.69)' // `linear-gradient(99deg, white 81%, ${this.publicRatingColor})`
                    : `linear-gradient(108deg, #063948 81%, ${this.publicRatingColor})`,
            });
        },
    },
    data() {
        return {
            hover: 0
        };
    },
})
</script>

<style scoped lang="scss">
.links-wrapper {
    display: flex;
    align-items: center;
    // justify-content: space-between;
    width: 100%;
    border-bottom-left-radius: 3px;
    border-bottom-right-radius: 3px;
    margin-top: -1px;

    .links-title {
        height: fit-content;
        min-width: 12.5%;
        width: 12.5%;
        max-width: 12.5%;
        color: white;
        transform: rotate(-90deg);
        // transform-origin: left top;
        // white-space: nowrap;
        background-color: transparent;
    }

    .link-cont {
        display: flex;
        align-items: center;
        justify-content: center;
        width: fit-content;
        min-width: 12.5%;
        height: 40px;
        background-color: black;
        cursor: pointer;
        text-decoration: none;
        color: white;

        &:hover {
            color: black;
            background-color: white;
        }
    }
}
</style>
