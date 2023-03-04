const Registry = require(`../../utility/registry`)
const {log} = require(`../../utility/log`);
const axios = require(`axios`)
const {getRefreshToken} = require(`./utility/get-refresh-token`)
const {jwtValidation} = require(`../../utility/jwt-validation`)
const {jwtReplaceToken} = require(`../../utility/jwt-replace-token`)

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
        if (req.params['id']) {
            const err = `No image id supplied.`

            console.error(err)
            res.status(400).send(err)
            return
        }

        try {
            await getRefreshToken()

            const imgurBaseURL = Registry.get(`ImgurBaseURL`)
            const userName = Registry.get(`ImgurUserName`)
            const formData = new URLSearchParams()
            const accessToken = Registry.get(`ImgurAccessToken`)
            const accessTokenValue = `Bearer ${accessToken}`
            const response = await axios({
                method: `DELETE`,
                headers: { 'Authorization': accessTokenValue },
                url: `${imgurBaseURL}image/${req.params['id']}`,
                redirect: `follow`,
                data: formData
            })

            if (200 !== response.status) {
                const err = `Request to delete an image for user ${userName} failed. Status ${response.status}`

                console.error(err)
                res.status(response.status).send(err)
                return
            }

            const result = await response.data
            const jwtReplaceTokenResult = await jwtReplaceToken(jwtValidationResult.jwtRegistryInfo)
    
            if (200 !== jwtReplaceTokenResult.status) {
                res.status(jwtReplaceTokenResult.status).send(jwtReplaceTokenResult.err)
                next && next(jwtReplaceTokenResult.err)
                return
            }

            const responseToClient = { jwt: jwtReplaceTokenResult.jwt, payload: { status: 200, data: result }}

            res.status(response.status).send(JSON.stringify(responseToClient))
        } catch (e) {
            const err = `Error deleting Imgur image. ${e.message}`

            console.error(err)
            res.status(500).send(err)
        }
    }
}