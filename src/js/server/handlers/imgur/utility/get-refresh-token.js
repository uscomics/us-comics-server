const {surrealDBChange} = require(`../../../database/surrealdb`)
const {IMGUR_REFRESH_TOKEN} = require(`../../../database/keys`)
const Registry = require(`../../../utility/registry`)
const axios = require('axios')

module.exports.getRefreshToken = async () => {
    try {
        const clientId = Registry.get(`ImgurClientId`)
        const clientSecret = Registry.get(`ImgurClientSecret`)
        const refreshToken = Registry.get(`ImgurRefreshToken`)
        const formData = new URLSearchParams()

        formData.append(`client_id`, clientId)
        formData.append(`client_secret`, clientSecret)
        formData.append(`refresh_token`, refreshToken)
        formData.append(`grant_type`, `refresh_token`)

        const response = await axios({
            method: `POST`,
            url: `https://api.imgur.com/oauth2/token`,
            redirect: `follow`,
            data: formData
        })
        if (200 !== response.status) {
            return { status: response.status, err: `Request to refresh Imgur token failed. Status ${response.status}` }
        }

        const newRefreshToken = response.data.refresh_token
        const db = Registry.get(`SurrealDBConnection`)
        const updateResult = await surrealDBChange(db, IMGUR_REFRESH_TOKEN, { value: newRefreshToken })

        if (updateResult.value !== newRefreshToken) {
            return { status: 500, err: `Request to refresh Imgur token failed. Could not update database.` }
        }
        Registry.register(`ImgurRefreshToken`, updateResult.value)
    } catch (err) {
        return { status: 500, err: `Request to refresh Imgur token failed. ${err.message}` }
    }
    return { status: 200 }
}
