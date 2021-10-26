<template>
    <div class="menu">
        <div class="logo" @click="goToHome"></div>

        <div class="search">
            <input type="text" placeholder="Search">
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useRouter } from 'vue-router';

export default defineComponent({
    setup() {
        const router = useRouter();

        const goToHome = () => {
            router.push('/');
        };

        return {
            goToHome
        };
    }
});
</script>

<style lang="scss" scoped>
.menu {
    z-index: 99;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: $menu-height;
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 0 10px;

    .logo {
        height: 35px;
        width: 35px;
        background-image: url('../assets/logo.png');
        background-size: 100%;
        cursor: pointer;
    }

    .search {
        input {
            height: 30px;
            border: 0;
            padding: 0 15px;
            border-radius: 10px;
            font-family: 'Montserrat-Medium';
            color: #FFFFFF;
            background-color: rgba(255, 255, 255, 0.2);
            outline: none;

            &::placeholder {
                color: #FFFFFF;
                opacity: 0.5;
            }
        }
    }
}
</style>
