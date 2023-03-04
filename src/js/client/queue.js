class Queue {
    static listeners = []
    static register(listener, message, callback) {
        Queue.listeners.push({ listener: listener, message: message, callback: callback })
    }
    static isRegistered(listener, message) {
        for (let loop = 0; loop < Queue.listeners.length; loop++) {
            let l = Queue.listeners[loop]

            if ((l.listener === listener) && (l.message === message)) {
                return true
            }
        }
        return false
    }
    static unregister(listener, message) {
        let removed = []

        for (let loop = 0; loop < Queue.listeners.length; loop++) {
            let l = Queue.listeners[loop]
            if ((l.listener === listener) && (l.message === message)) {
                removed.push(Queue.listeners.splice(loop, 1)[0])
            }
        }
        return removed
    }
    static unregisterAll() {
        Queue.listeners = []
    }
    static broadcast(message, data) {
        let receivers = []

        for (let loop = 0; loop < Queue.listeners.length; loop++) {
            let listenerRecord = Queue.listeners[loop]

            if (listenerRecord.message !== message) { continue }
            listenerRecord.callback(data)
            receivers.push(listenerRecord)
        }
        return receivers
    }
    static broadcastError(errorText) {
        return Queue.broadcast( Messages.ERROR, { message: errorText })
    }
    static broadcastNotification(notificationText) {
        return Queue.broadcast( Messages.NOTIFICATION, { message: notificationText })
    }
    static call(message, data) {
        let promises = []

        for (let loop = 0; loop < Queue.listeners.length; loop++) {
            let listenerRecord = Queue.listeners[loop]
            if (listenerRecord.message !== message) { continue }
            promises.push(listenerRecord.callback(data))
        }
        return Promise.all(promises)
    }
}