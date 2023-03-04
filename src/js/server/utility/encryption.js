const crypto = require ("crypto")
const algorithm = "aes-256-cbc"
const encoding = `utf-8`

function randomString(length) {
    const chars = `0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`
    let result = ''

    for (let i = length; i > 0; --i) { result += chars[Math.floor(Math.random() * chars.length)] }
    return result;
}
module.exports.generatePublicKey = () => {
    const publicKey = randomString(16)

    return publicKey
}
module.exports.generatePrivateKey = () => {
    const privateKey = randomString(32)

    return privateKey
}
module.exports.encrypt = (message, publicKey, privateKey) => {
    const cipher = crypto.createCipheriv(algorithm, Buffer.from(privateKey, encoding), Buffer.from(publicKey, encoding))
    let encryptedData = cipher.update(message, encoding, `hex`)

    encryptedData += cipher.final(`hex`)
    return encryptedData
}
module.exports.decrypt = (encryptedData, publicKey, privateKey) => {
    const decipher = crypto.createDecipheriv(algorithm, Buffer.from(privateKey, encoding), Buffer.from(publicKey, encoding))
    let decryptedData = decipher.update(encryptedData, `hex`, encoding)

    decryptedData += decipher.final(encoding)
    return decryptedData
}
