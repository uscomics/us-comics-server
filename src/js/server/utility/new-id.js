module.exports.getNewId = async (db, table) => {
    let queryResults = await db.query('SELECT id FROM type::table($tb)', { tb: table })
    let largestId = 0

    for (let result of queryResults[0].result) {
        const id = parseInt(result.id.split(`:`)[1])

        if (id > largestId) { largestId = id }
    }
    return largestId + 1
}
