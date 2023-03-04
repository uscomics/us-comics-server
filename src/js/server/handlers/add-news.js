const {surrealDBCreate} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)
const {fileMove: fileMove} = require(`../utility/file-move`)
const {getNewId} = require(`../utility/new-id`)
const {formidable} = require('formidable')
const fs = require('fs')
const path = require('path');
const {log} = require('../utility/log');

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const db = Registry.get(`SurrealDBConnection`)
        const authorizationHeader = req.get(`Authorization`)

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
        const jwtValidationResult = await jwtValidation(authorizationHeader)

        if (200 !== jwtValidationResult.status) {
            res.status(jwtValidationResult.status).send(jwtValidationResult.err)
            next && next(jwtValidationResult.err)
            return
        }

        const tempDir = `./src/images/temp`
        const finalDir = `./src/images`
        const formidableOptions = { uploadDir: tempDir }
        const form = formidable(formidableOptions)

        form.parse(req, async (parseError, parseFields, parseFiles) => {
            if (parseError) {
                const err = `400 Bad Request`

                console.error(err + `: Could not parse request.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.text) {
                const err = `400 Bad Request`

                console.error(err + `: Photo text not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename.filepath) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename.filepath not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            try {
                fs.existsSync(parseFiles.filename.filepath)
            } catch(e) {
                const err = `500 Internal Server Error`

                console.error(err + `: Photo ${parseFiles.filename.filepath} not uploaded.`)
                res.status(500).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename.originalFilename) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename.originalFilename not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.title) {
                const err = `400 Bad Request`
    
                console.error(err + `: Title not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
    
            try {
                const newId = await getNewId(db, entry.args.table)
                const newFileExtension = path.extname(parseFiles.filename.originalFilename)
                const newRecordId = `${entry.args.table}:${newId}`
                const newFileName = `news${newId}${newFileExtension}`
                const newPhotoRecord = { title: parseFields.title, text: parseFields.text, file: newFileName }
                const createResult = await surrealDBCreate(db, newRecordId, newPhotoRecord)
        
                fileMove(parseFiles.filename.filepath, `${finalDir}/${newFileName}`, async () => {
                    const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)
        
                    if (200 !== jwtReplaceTokenResult.status) {
                        res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
                        next && next(jwtReplaceTokenResult.err)
                        return
                    }
    
                    let response = { jwt: jwtReplaceTokenResult.jwt, payload: { status: 200, newRecord: createResult }}
    
                    res.status(200).send(JSON.stringify(response))
                })
            } catch (e) {
                const err = `500 Internal Server Error`
    
                console.error(err + `: Database operation failed.`)
                res.status(500).send(err)
                next && next(err)
                return
            }

        })
    }
}