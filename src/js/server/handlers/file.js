var path = require('path');
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const options = { root: path.join(`.`, '/src') }
        let file = entry.args.file

        for (const key in req.params) {
            const value = req.params[key]

            file.replace(`:${key}`, value)
        }

        res.sendFile(`${file}`, options, (err) => { if (err) { next && next(err) } else { next && next() } })
    }
}
