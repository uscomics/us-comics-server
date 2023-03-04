// node server.js
// flyctl apps restart uscomicsserver
// flyctl launch
// flyctl deploy
//
// flyctl ips list -a uscomicsserver
// * Add A and AAAA records for site at DNS provider
// flyctl certs create -a uscomicsserver vincedrivesyou.com
// flyctl certs show -a uscomicsserver
// * Connect to https://uscomicsserver.com
const express = require("express")
const HandlerManager = require('./src/js/server/load-handlers')
const Surreal = require(`surrealdb.js`)
const {surrealDBSignIn, surrealDBUse} = require(`./src/js/server/database/surrealdb`)
const {populateDatabase} = require('./src/js/server/database/populate-db')
const Registry = require(`./src/js/server/utility/registry`)
const {encrypt, decrypt} = require(`./src/js/server/utility/encryption`)
const bodyParser = require('body-parser')
const dotenv = require(`dotenv`)
const jwt = require(`jsonwebtoken`)

const initializeDatabase = async () => {
    // const db = new Surreal.default(`http://127.0.0.1:8000/rpc`)
    const db = new Surreal.default(`wss://vdydb.fly.dev/rpc`)

    try {
        await surrealDBSignIn(db, `root`, `root`)
        await surrealDBUse(db, `test`, `test`)
        Registry.register(`SurrealDBConnection`, db)
        await populateDatabase(db)
        return true
    } catch (e) {
        return false
    }
}
const initializeKeys = async () => {
    try {
        const db = Registry.get(`SurrealDBConnection`)
        const publicKeyQuery = `SELECT * FROM key WHERE name = 'PublicKey';`
        const privateKeyQuery = `SELECT * FROM key WHERE name = 'PrivateKey';`
        const imgurClientIdQuery = `SELECT * FROM key WHERE name = 'ImgurClientId';`
        const imgurClientSecretQuery = `SELECT * FROM key WHERE name = 'ImgurClientSecret';`
        const imgurRefreshTokenQuery = `SELECT * FROM key WHERE name = 'ImgurRefreshToken';`
        const imgurAccessTokenQuery = `SELECT * FROM key WHERE name = 'ImgurAccessToken';`
        const publicKeyQueryResult = await db.query(publicKeyQuery)
        const privateKeyQueryResult = await db.query(privateKeyQuery)
        const imgurClientIdQueryResult = await db.query(imgurClientIdQuery)
        const imgurClientSecretQueryResult = await db.query(imgurClientSecretQuery)
        const imgurRefreshTokenQueryResult = await db.query(imgurRefreshTokenQuery)
        const imgurAccessTokenQueryResult = await db.query(imgurAccessTokenQuery)
    
        if (0 === publicKeyQueryResult[0].result.length) {
            console.error(`Could not select public key.`)
            return false
        }
        if (0 === privateKeyQueryResult[0].result.length) {
            console.error(`Could not select private key.`)
            return false
        }
        if (0 === imgurClientIdQueryResult[0].result.length) {
            console.error(`Could not select Imgur client id.`)
            return false
        }
        if (0 === imgurClientSecretQueryResult[0].result.length) {
            console.error(`Could not select Imgur client secret.`)
            return false
        }
        if (0 === imgurRefreshTokenQueryResult[0].result.length) {
            console.error(`Could not select Imgur refresh token.`)
            return false
        }
        if (0 === imgurAccessTokenQueryResult[0].result.length) {
            console.error(`Could not select Imgur access token.`)
            return false
        }

        const publicKey = publicKeyQueryResult[0].result[0].value
        const privateKey = privateKeyQueryResult[0].result[0].value
        const encryptedImgurClientId = imgurClientIdQueryResult[0].result[0].value
        const imgurClientId = decrypt(encryptedImgurClientId, publicKey, privateKey)
        const encryptedImgurClientSecret = imgurClientSecretQueryResult[0].result[0].value
        const imgurClientSecret = decrypt(encryptedImgurClientSecret, publicKey, privateKey)
        const encryptedImgurRefreshToken = imgurRefreshTokenQueryResult[0].result[0].value
        const imgurRefreshToken = decrypt(encryptedImgurRefreshToken, publicKey, privateKey)
        const encryptedImgurAccessToken = imgurAccessTokenQueryResult[0].result[0].value
        const imgurAccessToken = decrypt(encryptedImgurAccessToken, publicKey, privateKey)

        if (!publicKey) {
            console.error(`Could not get public key.`)
            return false
        }
        if (!privateKey) {
            console.error(`Could not get private key.`)
            return false
        }
        if (!imgurClientId) {
            console.error(`Could not get Imgur client id.`)
            return false
        }
        if (!imgurClientSecret) {
            console.error(`Could not get Imgur client secret.`)
            return false
        }
        if (!imgurRefreshToken) {
            console.error(`Could not get Imgur refresh token.`)
            return false
        }
        if (!imgurAccessToken) {
            console.error(`Could not get Imgur access token.`)
            return false
        }
        Registry.register(`PublicKey`, publicKey)
        Registry.register(`PrivateKey`, privateKey)
        Registry.register(`ImgurClientId`, imgurClientId)
        Registry.register(`ImgurClientSecret`, imgurClientSecret)
        Registry.register(`ImgurRefreshToken`, imgurRefreshToken)
        Registry.register(`ImgurAccessToken`, imgurAccessToken)
        Registry.register(`ImgurUserName`, `USComics`)
        Registry.register(`ImgurBaseURL`, `https://api.imgur.com/3/`)

        return true
    } catch (e) {
        return false
    }

    // Imgur Client ID: 52e134f54050164
    // Imgur Client Secret: 0118748ce1ed918aac530406b4fca0c7cf44c8d5
    // Imgur Refresh Token: 56eac5410cfe71698a249300b79f776a874a6910
    // Imgur Access Token: a6133a3958a4970a77d72f853afae31c172d9f71
    // https://api.imgur.com/oauth2/authorize?client_id=52e134f54050164&response_type=token&state=TEST_STATE


}
const initializeServer = async () => {
    const app = express();
    const handlerManager = new HandlerManager()
    const data = handlerManager.readHandlers(`./config/handlers.json`)

    app.use(express.static(__dirname + '/src'));
    app.use(bodyParser.json())
    dotenv.config()
    Date.prototype.addHours = function(numberOfHours){
        this.setHours(this.getHours() + numberOfHours);
        return this;
    }
    Date.prototype.subtractHours = function(numberOfHours){
        this.setHours(this.getHours() - numberOfHours);
        return this;
    }

    Registry.register(`JWT`, jwt)

    // Load config
    handlerManager.buildHandlers(data, app)

    const databaseResult = await initializeDatabase()

    if (!databaseResult) {
        console.error(`Could not connect to database.`)
        return
    }

    const keysResult = await initializeKeys()

    if (!keysResult) {
        console.error(`Could not initialize security keys.`)
        return
    }

    Registry.register(`AppBaseURL`, `https://us-comics-server.fly.dev/`)

     // Start server
    const port = process.env.PORT || `8080`

    app.listen(port, () => console.log(`App listening on port ${port}.`))    
}

initializeServer()


