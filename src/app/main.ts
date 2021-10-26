import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import ionicons from './plugins/ionicons';
import toTimer from './directives/toTimer';

createApp(App)
    .use(router)
    .use(ionicons)
    .directive('to-timer', toTimer)
    .mount('#app')
