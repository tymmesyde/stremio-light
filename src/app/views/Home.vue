<template>
    <View class="home">
        <button @click="openPlayer">PLAYER</button>
    </View>
</template>

<script lang="ts">
import { defineComponent } from '@vue/runtime-core';
import { useRouter } from 'vue-router';
import View from '@/components/View.vue';

export default defineComponent({
    components: {
        View
    },
    setup() {
        const router = useRouter();

        const openPlayer = () => {
            router.push('/player');
        };

        return {
            openPlayer
        };
    }
});
</script>
