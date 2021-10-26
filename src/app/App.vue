<template>
    <Menu/>
    <router-view/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Menu from '@/components/Menu.vue';

export default defineComponent({
    components: {
        Menu
    }
});
</script>


<style lang="scss">
@import url('assets/styles/main.scss');

#app {
    height: 100%;
    width: 100%;
    user-select: none;

    .view {
        background-color: $background-color;
    }
}
</style>
