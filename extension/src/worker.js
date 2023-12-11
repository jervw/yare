import initWasmModule, { hello_background } from '../lib/yare.js';


(async () => {
    await initWasmModule();
    hello_background();
})();
