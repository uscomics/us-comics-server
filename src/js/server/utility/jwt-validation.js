const Registry = require(`./registry`)
var atob = require('atob');

module.exports.jwtValidation = async (authorizationHeader) => {
    if (!authorizationHeader) {
        const err = `401 Unauthorized`

        console.error(err + `: No authorization header provided.`)
        return { status: 401, err }
    }

    const jwt = Registry.get(`JWT`)
    const jwtToken = authorizationHeader.substring(7, authorizationHeader.length - 1).trim()
    const jwtRegistryInfo = Registry.get(jwtToken)
    const now = new Date()
    let expires
    
    if (!jwtRegistryInfo) {
        const err = `401 Unauthorized`

        console.error(err + `: No jwtRegistryInfo found for user.`)
        return { status: 401, err }
    }
    try {
        expires = Date.parse(jwtRegistryInfo.expires)
    } catch(e) {
        const err = `401 Unauthorized`

        console.error(err + `: Could not parse JWT expiration.`)
        return { status: 401, err }
    }
    if (!jwt) {
        const err = `503 Service Unavailable`

        console.error(err + `: Javascript Web Token service not available.`)
        return { status: 503, err }
    }
    if (!authorizationHeader.startsWith("'Bearer ")) {
        const err = `401 Unauthorized`

        console.error(err + `: Authorization header has an invalid format.`)
        return { status: 401, err }
    }
    if (!jwtRegistryInfo) {
        const err = `401 Unauthorized`

        console.error(err + `: Unknown JWT token.`)
        return { status: 401, err }
    }
    if (now > expires) {
        const err = `401 Unauthorized`

        Registry.unregister(jwtToken)
        console.error(err + `: JWT token has expired.`)
        return { status: 401, err }
    }
    return { status: 200, err: null, jwtRegistryInfo }
}
