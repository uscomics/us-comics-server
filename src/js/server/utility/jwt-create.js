const Registry = require(`./registry`)

module.exports.jwtCreate = async (name) => {
    const db = Registry.get(`SurrealDBConnection`)
    const jwt = Registry.get(`JWT`)

    if (!db) {
        const err = `503 Service Unavailable`

        console.error(err + `: Database not available.`)
        return { status: 503, err }
    }
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

    const query = `SELECT * FROM user WHERE userName = '${name}';`
    const queryResult = await db.query(query)

    if (0 === queryResult[0].result.length) {
        const err = `401 Service Unavailable`

        console.error(err + `: User name "${name}" not found in database.`)
        return { status: 401, err }
    }

    const jwtSecretKey = process.env.JWT_SECRET_KEY
    const claims = { iss: `uscomics`, roles: queryResult[0].result[0].roles }
    const token = jwt.sign(claims, jwtSecretKey)
    const roles = queryResult[0].result[0].roles
    const image = queryResult[0].result[0].image? queryResult[0].result[0].image : `generic-avatar`
    const registryEntry = { expires: new Date().addHours(1), name, roles, image }
    const clientResponse = { token, roles, image, name: queryResult[0].result[0].name, title: queryResult[0].result[0].title }

    Registry.register(token, registryEntry)
    return { status: 200, clientResponse }
}
