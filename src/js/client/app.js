class App {
  static initializeApp() {
    document.addEventListener(`DOMContentLoaded`, async () => { 
      Loader.registerCustomTags()
      await Loader.loadIncludes()
      SlotManager.loadSlots()
      Queue.broadcast(Messages.SITE_LOADED, null)
    })
  }
  static setServer() {
    // const server = `http://localhost:8080/`
    const server = `https://uscomicsserver.fly.dev/`
    Registry.register(`Server`, server)  
  }
  static setJWT() {
    const params = new URLSearchParams(window.location.search)
    const token = params.get(`token`)
  
    JavascriptWebToken.storeCredentials(JSON.parse(token))
  }
  static setCSSDefaults() {
  
  }
}