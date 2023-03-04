const {surrealDBChange, surrealDBSelect} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const authorizationHeader = req.get(`Authorization`)
        const reply = req.body
        
        if (!db) {
            const err = `503 Service Unavailable`
            const result = { status: 503, err }

            console.error(err + `: Database not available.`)
            res.status(result.status).send(JSON.stringify(result))
            next && next(err)
            return
        }
        if (!authorizationHeader) {
            const err = `401 Unauthorized`
            const result = { status: 401, err }

            console.error(err + `: Authorization header not sent with request.`)
            res.status(result.status).send(err)
            next && next(err)
            return
        }
        if (!reply) {
            const err = `400 Bad Request`
            const result = { status: 400, err }

            console.error(err + `: No reply found in request body.`)
            res.status(result.status).send(err)
            next && next(err)
            return
        }

        const jwtValidationResult = await jwtValidation(authorizationHeader)

        if (200 !== jwtValidationResult.status) {
            res.status(jwtValidationResult.status).send(jwtValidationResult.err)
            next && next(jwtValidationResult.err)
            return
        }

        const date = new Date();
        const today = ((date.getMonth() > 8) ? (date.getMonth() + 1) : ('0' + (date.getMonth() + 1))) + '/' + ((date.getDate() > 9) ? date.getDate() : ('0' + date.getDate())) + '/' + date.getFullYear()
        let updateData = {}
        
        updateData.reply = reply.reply
        updateData.date = today
        await surrealDBChange(db, reply.id, updateData)

        const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)

        if (200 !== jwtReplaceTokenResult.status) {
            res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
            next && next(jwtReplaceTokenResult.err)
            return
        }

        let response = { jwt: jwtReplaceTokenResult.jwt, payload: null }
        res.status(200).send(JSON.stringify(response))
        next && next()
    }
}
