const path = require('path');

const appPath = path.resolve(__dirname, 'src/app');

module.exports = {
    outputDir: 'dist',
    pages: {
        index: {
            entry: path.join(appPath, 'main.ts'),
            template: path.join(appPath, 'public/index.html')
        }
    },
    css: {
        loaderOptions: {
            sass: {
                additionalData: '@import "src/app/assets/styles/variables.scss";'
            }
        }
    },
    chainWebpack: config => {
        config.module
            .rule('vue')
            .use('vue-loader')
            .tap(options => ({
                ...options,
                compilerOptions: {
                    isCustomElement: tag => tag === 'ion-icon'
                }
            }));

        config.resolve.alias.set('@', appPath);
    }
};