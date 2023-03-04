const Registry = require(`./registry`)
const {jwtCreate} = require(`./jwt-create`)

module.exports.jwtReplaceToken = async (jwtRegistryInfo) => {
    const jwt = Registry.get(`JWT`)

    if (!jwt) {
        const err = `503 Service Unavailable`

        console.error(err + `: Javascript Web Token service not available.`)
        return { status: 503, err }
    }
    if (!process.env.JWT_SECRET_KEY) {
        const err = `503 Service Unavailable`

        console.error(err + `: Javascript Web Token key not available.`)
        return { status: 503, err }
    }

    const jwtCreateResult = await jwtCreate(jwtRegistryInfo.name)

    if (200 !== jwtCreateResult.status) { return jwtCreateResult }
    Registry.unregister(jwtRegistryInfo.token)
    return { status: 200, jwt: jwtCreateResult.clientResponse, err: null }
}
