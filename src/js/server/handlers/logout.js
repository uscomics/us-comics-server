const Registry = require(`../utility/registry`)
const {jwtCreate} = require(`../utility/jwt-create`)
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const authorizationHeader = req.get(`Authorization`)

        if (!authorizationHeader) {
            const err = `404 Not Found`
            console.error(err + `: No Authorization header.`)
            res.status(404).send(err)
            next && next(err)
            return
        }

        const jwtToken = authorizationHeader.substring(7, authorizationHeader.length - 1).trim()

        Registry.unregister(jwtToken)

        res.status(200).send(JSON.stringify(jwtCreateResult.clientResponse))
        next && next()
    }
}
