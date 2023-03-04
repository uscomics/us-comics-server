const Registry = require(`../utility/registry`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)
const {formidable} = require(`formidable`)
const fs = require(`fs`)
const {log} = require(`../utility/log`)

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        const authorizationHeader = req.get(`Authorization`)

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
                return { status: 400, err }
            }
            if (!parseFields.albumId) {
                const err = `400 Bad Request`

                console.error(err + `: Photo albumId not provided.`)
                return { status: 400, err }
            }
            if (!parseFields.title) {
                const err = `400 Bad Request`

                console.error(err + `: Photo title not provided.`)
                return { status: 400, err }
            }
            if (!parseFiles.filename) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename not provided.`)
                return { status: 400, err }
            }
            if (!parseFiles.filename.filepath) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename.filepath not provided.`)
                return { status: 400, err }
            }
            try {
                fs.existsSync(parseFiles.filename.filepath)
            } catch(e) {
                const err = `500 Internal Server Error`

                console.error(err + `: Photo ${parseFiles.filename.filepath} not uploaded.`)
                return { status: 500, err }
            }
            if (!parseFiles.filename.originalFilename) {
                const err = `400 Bad Request`

                console.error(err + `: Photo filename.originalFilename not provided.`)
                return { status: 400, err }
            }
            
            const formData = new URLSearchParams()
            const title = parseFields.title
            const albumId = parseFields.albumId
            const filename = parseFiles.filename.originalFilename
            const description = parseFields.description
            const appBaseURL = Registry.get(`AppBaseURL`)

            formData.append(`title`, title)
            formData.append(`image`, `${appBaseURL}/image/${filename}`)
            formData.append(`name`, filename)
            formData.append(`album`, albumId)
            if (description) { formData.append(`description`, description) }

            const imgurBaseURL = Registry.get(`ImgurBaseURL`)
            const userName = Registry.get(`ImgurUserName`)
            const accessToken = Registry.get(`ImgurAccessToken`)
            const accessTokenValue = `Bearer ${accessToken}`
            const response = await axios({
                method: `POST`,
                headers: { 'Authorization': accessTokenValue },
                url: `${imgurBaseURL}upload}`,
                redirect: `follow`,
                data: formData
            })

            fs.unlinkSync(filename)
                
            const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)
        
            if (200 !== jwtReplaceTokenResult.status) {
                res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
                next && next(jwtReplaceTokenResult.err)
                return
            }

            let responseToClient = { jwt: jwtReplaceTokenResult.jwt, payload: { status: 200, newRecord: response.data }}

            res.status(200).send(JSON.stringify(responseToClient))
        })
    }
}