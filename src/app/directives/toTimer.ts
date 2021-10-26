export default {
    mounted(el: HTMLElement) {
        el.innerHTML = '0:00';
    },
    updated(el: HTMLElement, binding: { value: number; }) {
        const date = new Date(binding.value * 1000);
        const sec = date.getUTCSeconds();
        const min = date.getUTCMinutes();
        const hours = date.getUTCHours();

        el.innerHTML = `${hours > 0 ? hours + ':' : ''}${hours && min < 10 ? '0' + min : min}:${sec < 10 ? '0' + sec : sec}`;
    }
}