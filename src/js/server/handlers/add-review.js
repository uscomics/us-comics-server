const {surrealDBQuery, surrealDBCreate} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const review = req.body
        
        if (!db) {
            const err = `503 Service Unavailable`
            const result = { status: 503, err }

            console.error(err + `: Database not available.`)
            res.status(result.status).send(JSON.stringify(result))
            next && next(err)
            return
        }
        if (!review) {
            const err = `400 Bad Request`
            const result = { status: 400, err }

            console.error(err + `: No review found in request body.`)
            res.status(result.status).send(err)
            next && next(err)
            return
        }

        const countResult = await surrealDBQuery(db, `SELECT * FROM type::table($tb)`, {tb: `review`, })
        const replyCount = countResult[0].result.length
        const newReplyId = `review:${replyCount + 1}`
        const date = new Date();
        const today = ((date.getMonth() > 8) ? (date.getMonth() + 1) : ('0' + (date.getMonth() + 1))) + '/' + ((date.getDate() > 9) ? date.getDate() : ('0' + date.getDate())) + '/' + date.getFullYear()
        
        review.date = today
        await surrealDBCreate(db, newReplyId, review)

        let response = { payload: { newId: newReplyId }}
        res.status(200).send(JSON.stringify(response))
        next && next()
    }
}
