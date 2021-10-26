import { reactive, computed } from "vue";

const state = reactive({
    visible: false,
    paused: true,
    volume: 0,
    time: 0,
    duration: 0
});

setInterval(() => {
    state.visible = window.player.visible;
    state.paused = window.player.paused;
    state.volume = window.player.volume;
    state.time = window.player.time;
    state.duration = window.player.duration;
}, 100);

export default () => computed(() => state);