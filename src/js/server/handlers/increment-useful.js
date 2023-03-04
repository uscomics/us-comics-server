const {surrealDBChange, surrealDBSelect} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const requestBody = req.body
        
        if (!db) {
            const err = `503 Service Unavailable`
            const result = { status: 503, err }

            console.error(err + `: Database not available.`)
            res.status(result.status).send(JSON.stringify(result))
            next && next(err)
            return
        }
        if (!requestBody) {
            const err = `400 Bad Request`
            const result = { status: 400, err }

            console.error(err + `: No reply found in request body.`)
            res.status(result.status).send(err)
            next && next(err)
            return
        }

        let updateData = {}
        let review = await surrealDBSelect(db, requestBody.id)
        
        review = review[0]
        if (!review.usefulCount) { updateData.usefulCount = 1 }
        else { updateData.usefulCount = review.usefulCount + 1 }
        await surrealDBChange(db, requestBody.id, updateData)

        res.status(200).send(`OK`)

        next && next()
    }
}
