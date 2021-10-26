<template>
    <div class="view" :class="{ 'transparent': transparent }">
        <slot></slot>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: {
        transparent: Boolean
    }
});
</script>


<style lang="scss" scoped>
.view {
    position: relative;
    height: 100%;
    width: 100%;
    padding-top: $menu-height;
    background-color: #0B0824;

    &.transparent {
        background-color: transparent !important;
    }
}
</style>