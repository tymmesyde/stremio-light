<template>
    <View :transparent="player.visible" class="player">
        <div class="controls">
            <input type="range" class="seek-bar" min="0" :max="player.duration" :value="player.time" @input="setTimePosition(Number($event.target.value))">

            <div class="panel">
                 <div class="button play" @click="togglePlayPause">
                    <ion-icon name="play" v-if="player.paused"></ion-icon>
                    <ion-icon name="pause" v-else></ion-icon>
                </div>
                <div class="time">
                    <span v-to-timer="player.time"></span> /
                    <span v-to-timer="player.duration"></span>
                </div>
            </div>

            <div class="panel">
                <div class="button volume">
                    <ion-icon name="volume-medium-outline"></ion-icon>
                    <input type="range" class="volume-bar" min="0" max="100" :value="player.volume" @input="setVolume(Number($event.target.value))">
                </div>
                <div class="button fullscreen" @click="toggleFullScreen">
                    <ion-icon name="expand" v-if="!fullscreen"></ion-icon>
                    <ion-icon name="contract" v-else></ion-icon>
                </div>
            </div>
        </div>
    </View>
</template>

<script lang="ts">
import { defineComponent, onMounted, onUnmounted, ref } from 'vue';
import View from '@/components/View.vue';
import usePlayerStore from '../store/player';

export default defineComponent({
    components: {
        View
    },
    setup() {
        const player = usePlayerStore();
        const fullscreen = ref(false);

        const togglePlayPause = async () => {
            player.value.paused ? await window.rpc.call('player.play') : await window.rpc.call('player.pause');
        };

        const setVolume = async (value: number) => {
            await window.rpc.call('player.setVolume', value);
        };

        const setTimePosition = async (value: number) => {
            await window.rpc.call('player.setTimePosition', value);
        };

        const toggleFullScreen = async () => {
            fullscreen.value = !fullscreen.value;
            await window.rpc.call('setFullscreen', fullscreen.value);
        };

        onMounted(async () => {
            await window.rpc.call('player.attach');
            await window.rpc.call('player.loadFile', 'http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_1080p_30fps_normal.mp4');

            document.ondblclick = () => toggleFullScreen();
            document.onkeydown = ({ code }) => {
                if (code === "Space") togglePlayPause();
                if (code === "ArrowLeft") setTimePosition(player.value.time - 10);
                if (code === "ArrowRight") setTimePosition(player.value.time + 10);
            };
            document.onwheel = ({ deltaY }) => {
                deltaY > 0 ? setVolume(player.value.volume - 10) : setVolume(player.value.volume + 10);
            };
        });

        onUnmounted(async () => {
            await window.rpc.call('player.stop');
            await window.rpc.call('setFullscreen', false);
        });

        return {
            player,
            fullscreen,
            setVolume,
            setTimePosition,
            togglePlayPause,
            toggleFullScreen
        };
    }
});
</script>

<style lang="scss" scoped>
.player {
    background-color: black !important;

    .controls {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        justify-content: space-between;
        height: 60px;
        padding: 0 20px;
        padding-bottom: 10px;
        background: linear-gradient(0deg, rgba(0, 0, 0, 0.5) 0%, rgba(0, 0, 0, 0) 100%);
        opacity: 1;
        transition: opacity 0.1s ease-in-out;

        .panel {
            display: flex;
            align-items: center;
            gap: 15px;
        }

        .button {
            display: flex;
            justify-content: center;
            text-align: center;
            width: 30px;
            cursor: pointer;

            ion-icon {
                font-size: 30px;
                color: white;
            }

            &.play, &.pause {
                width: 40px;

                ion-icon {
                    font-size: 40px;
                }
            }
        }

        .time {
            font-family: 'Montserrat-Regular';
            font-size: 15px;
            color: white;
        }

        .volume {
            position: relative;

            &:hover {
                input {
                    opacity: 1;
                }
            }

            input {
                position: absolute;
                width: 100px;
                left: -40px;
                top: -60px;
                transform: rotate(-90deg);
                transform-origin: center;
                opacity: 0;
                transition: 0.1s ease-in-out;
            }
        }

        .seek-bar {
            position: absolute;
            bottom: 0;
            left: 0;
            width: 100%;
            margin: 0;
        }
    }

    &:hover {
        .controls {
            opacity: 1;
        }
    }
}
</style>
