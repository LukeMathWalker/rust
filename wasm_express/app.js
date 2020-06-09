const express = require('express')
const { get_query_plan } = require('./pkg/wasm_express')

const app = express()
const port = 3000

app.get('/', (req, res, next) => {
    res.send(JSON.stringify(get_query_plan()));
})

app.listen(port, () => console.log(`Example app listening at http://localhost:${port}`))
