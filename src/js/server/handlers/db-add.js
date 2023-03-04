const {surrealDBCreate} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)
const {getNewId} = require(`../utility/new-id`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const authorizationHeader = req.get(`Authorization`)
        let originalRequest = req.body
        let processedRequest = originalRequest

        if (!db) {
            const err = `503 Service Unavailable`
            console.error(err + `: Surreal DB`)
            res.status(503).send(err)
            next && next(err)
            return
        }
        if (!entry?.args?.table) {
            const err = `503 Service Unavailable`
            console.error(err + `: Missing entry.args.table.`)
            res.status(503).send(err)
            next && next(err)
            return
        }
        if (!originalRequest) {
            const err = `400 Bad Request`
            const result = { status: 400, err }

            console.error(err + `: No data found in request body.`)
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

        if (entry.args.preprocessor) {
            const {preprocessor} = require(entry.args.preprocessor)
            const preprocessorResult = await preprocessor(originalRequest)

            if (200 !== preprocessorResult.status) {
                res.status(preprocessorResult.status).send(preprocessorResult.err)
                next && next(preprocessorResult.err)
                return
            } else {
                processedRequest = preprocessorResult.processedRequest
            } 
        }

        if (entry.args.validator) {
            const {validator} = require(entry.args.validator)
            const validatorResult = await validator(processedRequest)

            if (200 !== validatorResult.status) {
                res.status(validatorResult.status).send(validatorResult.err)
                next && next(validatorResult.err)
                return
            }    
        }

        const newId = await getNewId(db, entry.args.table)
        const newRecordId = `${entry.args.table}:${newId}`
        
        await surrealDBCreate(db, newRecordId, processedRequest)

        if (entry.args.postprocessor) {
            const {postprocessor} = require(entry.args.postprocessor)
            const postprocessorResult = await postprocessor(originalRequest, processedRequest, response)

            if (200 !== postprocessorResult.status) {
                res.status(postprocessorResult.status).send(postprocessorResult.err)
                next && next(postprocessorResult.err)
                return
        }

        const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)

        if (200 !== jwtReplaceTokenResult.status) {
            res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
            next && next(jwtReplaceTokenResult.err)
            return
        }

        let response = { jwt: jwtReplaceTokenResult.jwt, payload: { response: `Operation completed.` }}

        res.status(200).send(JSON.stringify(response))
    }
}