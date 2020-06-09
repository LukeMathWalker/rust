const express = require('express')
const rust = import('./pkg').catch(console.error);

const app = express()
const port = 3000

app.get('/', (req, res, next) => {
    rust
        .then(m => {
            qp = m.get_query_plan();
            res.send(JSON.stringify(qp));
        })
        .catch(next);
})

app.listen(port, () => console.log(`Example app listening at http://localhost:${port}`))
