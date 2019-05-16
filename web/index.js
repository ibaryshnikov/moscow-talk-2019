import init, { doMath } from './pkg/web.js'

window.addEventListener('load', async () => {
    await init('./pkg/web_bg.wasm');

    const { a, b } = doMath();
    console.log('a', a);
    console.log('b', b);
});
