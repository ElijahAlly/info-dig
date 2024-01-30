<template>
    <div
        class="gradient-btn-wrapper"
        :style="btnWrapperStyle"
        @mouseover="hover=true"
        @mouseleave="hover=false"
    >
        <button
            @click="$props.actionOnClick"
            class="gradient-btn"
            :style="btnInnerStyle"
        >
            {{ $props.text }}
        </button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'GradientButton',
    data() {
        return {
            hover: false
        };
    },
    props: {
        actionOnClick: {
            type: Function,
            required: true
        },
        text: {
            type: String,
            required: true
        },
        textColor: {
            type: String,
            default: 'aliceblue'
        },
        colorsInnerInitial: {
            type: String,
            default: '#063948 30%, #107fca'
        },
        colorsInnerHover: {
            type: String,
            default: '#063948 3%, #107fca'
        },
        colorsWrapper: {
            type: String,
            default: 'transparent 66%, gray'
        },
        initialAngle: {
            type: String,
            default: '158deg'
        },
        hoverAngle: {
            type: String,
            default: '148deg'
        }
    },
    computed: {
        btnInnerStyle() {
            return ({
                color: this.textColor,
                background: this.hover
                    ? `linear-gradient(${this.$props.hoverAngle}, ${this.colorsInnerHover})`
                    : `linear-gradient(${this.$props.initialAngle}, ${this.colorsInnerInitial})`,
            });
        },
        btnWrapperStyle() {
            return ({
                background: `linear-gradient(${this.$props.initialAngle}, ${this.colorsWrapper})`,
            })
        }
    }
})
</script>

<style scoped lang="scss">
.gradient-btn-wrapper {
    width: fit-content;
    height: fit-content;
    border-radius: 6px;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;

    .gradient-btn {
        outline: none;
        text-decoration: none;
        width: fit-content;
        height: 33px;
        border-width: 0;
        border-radius: 6px;
        cursor: pointer;
        font-size: small;
        font-weight: bold;
        padding: 6px;
    }

    &:hover {
        rotate: -12deg 21 3 1;
    }
}
</style>