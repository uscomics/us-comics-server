// ONLY USED TO SET UP A FRESH DATABASE.
// surreal start --log debug --user root --pass root memory
// fly apps restart uscomicsdb
module.exports = {
  populateDatabase: async function() {
    const Surreal = require(`surrealdb.js`)
    const {surrealDBSignIn, surrealDBUse, surrealDBCreate, surrealDBChange, surrealDBSelect, surrealDBQuery, surrealDBDelete} = require(`./surrealdb`)
    const Registry = require(`../utility/registry`)

    let db = Registry.get(`SurrealDBConnection`)
    
    if (!db) {
      console.error(`No SurrealDB connection found in register.`)
      return
    }

    try {
      await surrealDBDelete(db, `key`)
      await surrealDBCreate(db, `key:0`, {
        name: `PublicKey`,
        value: `bQCukzCb8nK1J9mR`
      })
      await surrealDBCreate(db, `key:1`, {
        name: `PrivateKey`,
        value: `BOf71yQTB2IfbnnLvSZizAujTKWdfaqx`
      })
      await surrealDBCreate(db, `key:2`, {
        name: `ImgurClientId`,
        value: `7791f07b5343fa9f679bb26f6b567b7a`
      })
      await surrealDBCreate(db, `key:3`, {
        name: `ImgurClientSecret`,
        value: `6d4f4da11e479eeb76d32ecaa273de7e958684ec54a1b511263cfd352ac2609ece5294364a8d7c6bc13ad470afb2a8bc`
      })
      await surrealDBCreate(db, `key:4`, {
        name: `ImgurRefreshToken`,
        value: `766a89445d6acae1fb28c0d736ff54bdacc5d3900de9dff34fc9033ea17d7d309f2481ef596e60b90db19f2f26bd5381`
      })
      await surrealDBCreate(db, `key:5`, {
        name: `ImgurAccessToken`,
        value: `579ff67033a8ba88fb691ac32b2203948ce552f128ebe07205c164e8a35b989b51a6b1dfabee9ae58e82f7c50f256865`
      })

      await surrealDBDelete(db, `text`)
      await surrealDBCreate(db, `text:0`, {
        for: `about`,
        text: `Vince Drives You is a Luxury SUV Transportation Service serving the Valley of the Sun. Advance reservations are required. I have an on-time pickup guarantee which means if you're not picked up on time you get a $50 discount on that trip, not a future credit. You can't do better anywhere.
        \n\nIf you are looking for a spacious, clean and comfortable trip then calling me is the only number you need to know. Rates start at $60 and hourly packages are available for your special events. There is never a surge charge, you will know your full fee when your trip is confirmed regardless of how far in advance your service is booked. I now have a Mercedes Sprinter with an 11 passenger capacity also available for larger party pick ups at the airport or for special events.`,
      })
      await surrealDBCreate(db, `text:1`, {
        for: `reviews`,
        text: `These are all genuine reviews from real customers. I can't add to them or edit them. I can only edit my replies, because occasionally I make a mistake.\n\nWe very much enjoy the feedback our customers give us. If you ride with us, feel free to let us know how we did.\n\nVince`,
      })

      await surrealDBDelete(db, `user`)
      await surrealDBCreate(db, `user:0`, {
        userName: `Admin`,
        name: {
          first: `Admin`,
          last: `Admin`
        },
        title: `Admin`,
        password: `Admin`,
        roles: [`Admin`],
      })
      await surrealDBCreate(db, `user:1`, {
        userName: `Dave`,
        name: {
          first: `Dave`,
          last: `L`
        },
        title: `Business Owner`,
        password: `Vincent`,
        image: `https://s3-media0.fl.yelpcdn.com/buphoto/P5CU_mkQeRZtDwo6g9Vpeg/30s.jpg`,
        roles: [`Admin`],
      })

      await surrealDBDelete(db, `photo`)
      await surrealDBCreate(db, `photo:0`, { text: `Enjoy a stress free trip to the red rocks of Sedona.`, file: `photo0.jpg` })

      await surrealDBDelete(db, `news`)
      await surrealDBCreate(db, `news:0`, { title: `Visiting the red rocks of Sedona.`, text: `Phoenix Sky Harbor to Sedona, up to 7 passengers plus luggage for only $240 one way. Call Vince at (602) 545-8557 to schedule your trip.`, file: `news0.jpg` })

      await surrealDBDelete(db, `review`)
      await surrealDBCreate(db, `review:1`, {
        name: {
          first: `Celeste`,
          last: `H`
        },
        date: `3/29/2022`,
        city: `Surprise`,
        state: `AZ`,
        stars: 5,
        usefulCount: 0,
        funnyCount: 0,
        coolCount: 0,
        review: `Our Uber driver cancelled our ride, we found Vince through Yelp and he not only answered our call at 4am but got us to the airport. I would highly recommend Vince's service, he is professional and friendly. Additionally he was very reasonable in price.`
      })
      await surrealDBCreate(db, `review:30`, {
        replyingTo: 1,
        image: `https://s3-media0.fl.yelpcdn.com/buphoto/P5CU_mkQeRZtDwo6g9Vpeg/30s.jpg`,
        name: {
          first: `Vincent`,
          last: `S`
        },
        date: `3/30/2022`,
        title: `Business Owner`,
        reply: `Thanks for your compliment, I’m glad I was able to help. It was a pleasure meeting you.`
      })

      const text = await surrealDBSelect(db, `text`)
      console.log(text.length)
      console.log(JSON.stringify(text))
      const photos = await surrealDBSelect(db, `photo`)
      console.log(photos.length)
      console.log(JSON.stringify(photos))
      const news = await surrealDBSelect(db, `news`)
      console.log(news.length)
      console.log(JSON.stringify(news))
      const users = await surrealDBSelect(db, `user`)
      console.log(users.length)
      console.log(JSON.stringify(users))
      const reviews = await surrealDBSelect(db, `review`)
      console.log(reviews.length)
      console.log(JSON.stringify(reviews))
      let countResult = await surrealDBQuery(db, `SELECT * FROM type::table($tb)`, {tb: `review`, })
      console.log(countResult[0].result.length)

    } catch (e) {
        console.error(`ERROR POPULATING DATABASE.`, e);
    }
  }
}
