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
    const r = document.querySelector(`:root`)
    var rs = getComputedStyle(r);
    const blue50 = rs.getPropertyValue(`--blue-50`)
    const black = rs.getPropertyValue(`--black`)
    r.style.setProperty(`--button-color`, black);
    r.style.setProperty(`--button-background-color`, blue50);
    r.style.setProperty(`--button-border-color`, blue50);
  
    const blueGray900 = rs.getPropertyValue(`--blue-gray-900`)
    const blueGray50 = rs.getPropertyValue(`--blue-gray-50`)
    const amber900 = rs.getPropertyValue(`--amber-500`)
    r.style.setProperty(`--background-color`, blueGray900)
    r.style.setProperty(`--primary-text-color`, blueGray50)
    r.style.setProperty(`--secondary-color`, amber900)
  
    r.style.setProperty(`--button-color`, blueGray900)
    r.style.setProperty(`--button-background-color`, amber900)
  }
}