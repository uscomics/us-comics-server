const Registry = require(`../../utility/registry`)
const {log} = require(`../../utility/log`)
const axios = require(`axios`)
const formidable = require(`formidable`)
const {jwtValidation} = require(`../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../utility/jwt-replace-token`)

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)
        
        try {
            const form = formidable({})
            const authorizationHeader = req.get(`Authorization`)
            const jwtValidationResult = await jwtValidation(authorizationHeader)

            if (200 !== jwtValidationResult.status) {
                res.status(jwtValidationResult.status).send(jwtValidationResult.err)
                next && next(jwtValidationResult.err)
                return
            }

            form.parse(req, async (parseError, parseFields, parseFiles) => {
                if (parseError) {
                    const err = `400 Bad Request`

                    console.error(err + `: Could not parse request.`)
                    res.status(400).send(err)
                    next && next(err)
                    return
                }
                if (!parseFields.title) {
                    const err = `400 Bad Request`

                    console.error(err + `: Album title not provided.`)
                    res.status(400).send(err)
                    next && next(err)
                    return
                }

                const formData = new URLSearchParams()
                const title = parseFields.title
                const description = parseFields.description

                formData.append(`title`, title)
                if (description) { formData.append(`description`, description) }

                const imgurBaseURL = Registry.get(`ImgurBaseURL`)
                const userName = Registry.get(`ImgurUserName`)
                const accessToken = Registry.get(`ImgurAccessToken`)
                const accessTokenValue = `Bearer ${accessToken}`
                const response = await axios({
                    method: `POST`,
                    headers: { 'Authorization': accessTokenValue },
                    url: `${imgurBaseURL}album}`,
                    redirect: `follow`,
                    data: formData
                })

                if (200 !== response.status) {
                    const err = `Request to add an album for user ${userName} failed. Status ${response.status}`

                    console.error(err)
                    res.status(response.status).send(err)
                    return
                }

                const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)
    
                if (200 !== jwtReplaceTokenResult.status) {
                    res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
                    next && next(jwtReplaceTokenResult.err)
                    return
                }
    
                const responseToClient = { jwt: jwtReplaceTokenResult.jwt, payload: { status: 200, newRecord: response.data }}

                res.status(response.status).send(JSON.stringify(responseToClient))
            })
        } catch (e) {
            const err = `Error adding Imgur album. ${e.message}`

            console.error(err)
            res.status(500).send(err)
        }
    }
}