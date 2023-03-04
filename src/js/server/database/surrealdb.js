module.exports = {
	surrealDBSignIn: async function (connection, user, pass) { await connection.signin({ user, pass, }) },
	surrealDBUse: async function (connection, namespace, database) { await connection.use(namespace, database) },
	surrealDBClose: async function (connection) { await connection.close() },
	surrealDBCreate: async function (connection, thing, data) {
		let created = await connection.create(thing, data)
		return created
	},
	surrealDBChange: async function (connection, thing, data) {
		let changed = await connection.change(thing, data)
		return changed
	},
	surrealDBSelect: async function (connection, thing) {
		let data = await connection.select(thing)
		return data
	},
	surrealDBQuery: async function (connection, query, options) {
		let data = await connection.query(query, options)
		return data
	},
	surrealDBDelete: async function (connection, thing) { await connection.delete(thing) }
}