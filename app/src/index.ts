import init, { do_thing } from "../../lib/pkg/wasm_poc.js";

await init();

const column = new Float64Array(1024 * 1024);
for (let i = 0; i < column.length; i++) {
  column[i] = Math.random() * Number.MAX_SAFE_INTEGER;
}

const start = performance.now();

const buffer = do_thing(column);

const end = performance.now();

console.log(buffer);
console.log(`WASM took ${(end - start).toFixed(2)} ms`);
