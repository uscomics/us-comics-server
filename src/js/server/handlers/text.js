const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)
        res.send(entry.args.text)
        next && next()
    }
}
