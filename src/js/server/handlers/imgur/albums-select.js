const Registry = require(`../../utility/registry`)
const {log} = require(`../../utility/log`)
const axios = require(`axios`)

module.exports = (entry) => {
    return async (req, res, next) => {
        log(entry)

        try {
            const imgurBaseURL = Registry.get(`ImgurBaseURL`)
            const clientId = Registry.get(`ImgurClientId`)
            const userName = Registry.get(`ImgurUserName`)
            const formData = new URLSearchParams()
            const clientIdValue = `Client-ID ${clientId}`
            const response = await axios({
                method: `GET`,
                headers: { 'Authorization': clientIdValue },
                url: `${imgurBaseURL}account/${userName}/albums`,
                redirect: `follow`,
                data: formData
            })

            if (200 !== response.status) {
                const err = `Request to fetch album list for user ${userName} failed. Status ${response.status}`

                console.error(err)
                res.status(response.status).send(err)
                return
            }

            const result = await response.data
            
            res.status(response.status).send(JSON.stringify(result))
            next && next()
        } catch (e) {
            const err = `Error retrieving Imgur album list. ${e.message}`

            console.error(err)
            res.status(500).send(err)
        }
    }
}