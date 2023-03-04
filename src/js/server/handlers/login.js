const Registry = require(`../utility/registry`)
const {jwtCreate} = require(`../utility/jwt-create`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const name = req.body.name
        const password = req.body.password

        if (!db) {
            const err = `503 Service Unavailable`

            console.error(err + `: Database not available.`)
            res.status(503).send(err)
            next && next(err)
            return
        }
        if (!name || !password) {
            const err = `401 Unauthorized`

            console.error(err + `: User name or password not supplied by login request.`)
            res.status(401).send(err)
            next && next(err)
            return
        }

        const query = `SELECT * FROM user WHERE userName = '${name}';`
        const queryResult = await db.query(query)
        const recordsFound = queryResult[0]?.result?.length

        if (!recordsFound) {
            const err = `401 Unauthorized`

            console.error(err + `: User name "${name}" not found in database.`)
            res.status(401).send(err)
            next && next(err)
            return
        }
        if (queryResult[0].result[0].password !== password) {
            const err = `401 Unauthorized`

            console.error(err + `: User "${name}" provided the wrong password.`)
            res.status(401).send(err)
            next && next(err)
            return
        }

        const jwtCreateResult = await jwtCreate(name)

        if (200 !== jwtCreateResult.status) {
            res.status(jwtCreateResult.status).send(jwtCreateResult.err)
            next && next(jwtCreateResult.err)
            return
        }

        res.status(200).send(JSON.stringify(jwtCreateResult.clientResponse))
        next && next()
    }
}
