# WASM bridge prototype

## How to make this work

```shell script
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cd wasm_express
wasm-pack build -t nodejs
node app.js
```
