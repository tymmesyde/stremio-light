/* eslint-disable */
declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    const component: DefineComponent<{}, {}, any>
    export default component
}

declare module globalThis {
    interface Window {
        rpc: {
            call: (name: string, argument?: string | number | boolean) => Promise<void>;
        };

        player : {
            visible: boolean;
            paused: boolean;
            volume: number;
            time: number;
            duration: number;
        };
    }
}