const {surrealDBChange, surrealDBSelect} = require(`../database/surrealdb`)
const Registry = require(`../utility/registry`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)
const {fileMove} = require(`../utility/file-move`)
const {formidable} = require('formidable')
const fs = require('fs')
const path = require('path');
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

        const tempDir = `./src/images/temp`
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
            if (!parseFields.userName) {
                const err = `400 Bad Request`
    
                console.error(err + `: User userName not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.password) {
                const err = `400 Bad Request`
    
                console.error(err + `: User password not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.roles) {
                const err = `400 Bad Request`
    
                console.error(err + `: User roles not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.title) {
                const err = `400 Bad Request`
    
                console.error(err + `: User title not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.firstName) {
                const err = `400 Bad Request`
    
                console.error(err + `: User firstName not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.lastName) {
                const err = `400 Bad Request`
    
                console.error(err + `: User lastName not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename) {
                const err = `400 Bad Request`
    
                console.error(err + `: User filename not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename.filepath) {
                const err = `400 Bad Request`
    
                console.error(err + `: User filename.filepath not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            try {
                fs.existsSync(parseFiles.filename.filepath)
            } catch(e) {
                const err = `500 Internal Server Error`
    
                console.error(err + `: User ${parseFiles.filename.filepath} not uploaded.`)
                res.status(500).send(err)
                next && next(err)
                return
            }
            if (!parseFiles.filename.originalFilename) {
                const err = `400 Bad Request`
    
                console.error(err + `: User filename.originalFilename not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            if (!parseFields.id) {
                const err = `400 Bad Request`
    
                console.error(err + `: User id not provided.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
    
            let oldRecord = await surrealDBSelect(db, parseFields.id)
            const finalDir = `./src/images`
    
            if (1 !== oldRecord.length) {
                const err = `400 Bad Request`
    
                console.error(err + `: No existing user record found for update.`)
                res.status(400).send(err)
                next && next(err)
                return
            }
            oldRecord = oldRecord[0]
            try { if (fs.existsSync(`${finalDir}/${oldRecord.file}`)) { fs.unlinkSync(`${finalDir}/${oldRecord.file}`) } }
            catch(e) {
                const err = `500 Internal Server Error`
    
                console.error(err + `: Could not delete ${finalDir}/${oldRecord.file}.`)
                res.status(500).send(err)
                next && next(err)
                return
            }
    
            const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)
    
            if (200 !== jwtReplaceTokenResult.status) {
                res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
                next && next(jwtReplaceTokenResult.err)
                return
            }
    
            const fileNumber = parseInt(parseFields.id.split(`:`)[1])
            const newFileExtension = path.extname(parseFiles.filename.originalFilename)
            const newFileName = `user${fileNumber}${newFileExtension}`
            const updateData = { title: parseFields.title, text: parseFields.text, file: newFileName, id: parseFields.id }
            const createResult = await surrealDBChange(db, parseFields.id, updateData)
    

            fileMove(parseFiles.filename.filepath, `${finalDir}/${newFileName}`, () => {
                let response = { jwt: jwtReplaceTokenResult.jwt, payload: { status: 200, newRecord: updateData }}
    
                res.status(200).send(JSON.stringify(response))
                next && next()
            })
        })
    }
}
