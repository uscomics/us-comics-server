const Registry = require(`../src/js/registry`)

document.addEventListener('DOMContentLoaded', async () => {
    suite(`Test Registry`, `Ensure Registry is working.`, [
        await test (`Register and unregister`, `Ensure we can register and unregister listeners`, [async () => {
            let results = []
            const a = {}
            const b = {}

            Registry.register("A", a)
            Registry.register("B", b)

            assert(Registry.get("A") === a,                                     `One object properly placed in registry.`, results)
            assert(Registry.get("B") === b,                                     `Two objects properly placed in registry.`, results)

            let r = Registry.unregister("A");

            assert(Registry.isRegistered("A") === false,                        `One object properly unregistered.`, results)
            assert(Registry.get("A") === null,                                  `Trying to get an unregistered object returns null.`, results)
            assert(r === a,                                                     `Unregistered object return by unregister call.`, results)
            assert(Registry.unregister("A") === null,                           `Trying to unregister an object not in the registry returns null.`, results)
            assert(Registry.isRegistered("B") === true,                         `Unregistering an object does not unregister other objects.`, results)

            Registry.unregisterAll()
            
            assert(Registry.get("B") === null,                                  `Trying to get an object from an emprty registry returns null.`, results)
    
            return results
        }]),
    ])
})
