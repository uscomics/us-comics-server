module.exports = class Registry {
    static registry = [];

    static register(name, object) {
        Registry.registry.unshift({object: object, name: name})
    }

    static unregister(name) {
        for (let loop = 0; loop < Registry.registry.length; loop++) {
            if (Registry.registry[loop].name === name) {
                let object = Registry.registry[loop].object;
                Registry.registry.splice(loop, 1);
                return object;
            }
        }
        return null;
    }

    static unregisterAll() {
        Registry.registry = [];
    }

    static isRegistered(name) {
        for (let loop = 0; loop < Registry.registry.length; loop++) {
            if (Registry.registry[loop].name === name) return true;
        }
        return false;
    }

    static get(name) {
        for (let loop = 0; loop < Registry.registry.length; loop++) {
            if (Registry.registry[loop].name === name) return Registry.registry[loop].object;
        }
        return null;
    }
}