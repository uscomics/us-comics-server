class TestApp {
  static initializeApp() {
    document.addEventListener(`DOMContentLoaded`, async () => { 
    Loader.registerCustomTags()
    await Loader.loadIncludes(true)
    SlotManager.loadSlots()
    Queue.broadcast(Messages.SITE_LOADED, null)
    })
  }
}