// TODO: two-way binding.
class Component {
    static getFragment = (componentClass) => {
        if (!componentClass) { 
            console.error(`getFragment: No component fragment id provided.`)
            return null 
        }
        if (!window?.$components?.fragmentRegistry?.has(componentClass)) { 
            console.error(`getFragment: Component fragment ${componentClass} is not registered.`)
            return null 
        }
        
        let fragment = window.$components.fragmentRegistry.get(componentClass)
        return fragment
    }
    static isObjectRegistered = (componentObjectId) => {
        if (!componentObjectId) { 
            console.error(`getObject: No component object id provided.`)
            return null 
        }
        return window?.$components?.objectRegistry?.has(componentObjectId)
    }
    static getObject = (componentObjectId) => {
        if (!componentObjectId) { 
            console.error(`getObject: No component object id provided.`)
            return null 
        }
        if (!window?.$components?.objectRegistry?.has(componentObjectId)) { 
            console.error(`getObject: Component object ${componentObjectId} is not registered.`)
            return null 
        }
        
        let componentObjectInfo = window.$components.objectRegistry.get(componentObjectId)
        return componentObjectInfo.componentObject
    }
    static createComponentInclude = (includeIn, src, componentClass, componentId, props, vars) => {
        let newInclude = document.createElement(`include-html`, { })

        newInclude.setAttribute(`include-in`, includeIn)
        newInclude.setAttribute(`src`, src)
        newInclude.setAttribute(`component-class`, componentClass)
        newInclude.setAttribute(`component-id`, componentId)

        if (props) {
            let newIncludeProps = document.createElement(`include-props`, { })

            newIncludeProps.innerText = JSON.stringify(props)
            newInclude.appendChild(newIncludeProps)
        }
        if (vars) {
            let newIncludeVars = document.createElement(`include-vars`, { })

            newIncludeVars.innerText = JSON.stringify(vars)
            newInclude.appendChild(newIncludeVars)
        }
        return newInclude
    }
    className() {return this.constructor.name }
    initialize(id) { 
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_INITIALIZATION, this)
        this.id = id
        Queue.register(this, ComponentLifecycle.msgs.COMPONENT_AFTER_MOUNT, (message) => {
            const childComponents = window.$components.childComponentRegistry.get(this.className())

            if (!message.id || !childComponents ) { return }

            const objectInfo = window.$components.objectRegistry.get(this.id)

            if (!objectInfo) { return }
            if (this.haveChildrenMounted() && !objectInfo.hasBroadcastChildrenMounted) {
                objectInfo.hasBroadcastChildrenMounted = true
                window.$components.objectRegistry.set(this.id, objectInfo)
                this.onChildrenMounted()
            }
            if (this.haveDescendantsMounted() && !objectInfo.hasBroadcastDescendantsMounted) {
                objectInfo.hasBroadcastDescendantsMounted = true
                window.$components.objectRegistry.set(this.id, objectInfo)
                this.onDescendantsMounted()
            }

            if (!childComponents.length) {
                if (!objectInfo.hasBroadcastChildrenMounted) {
                    objectInfo.hasBroadcastChildrenMounted = true
                    window.$components.objectRegistry.set(this.id, objectInfo)
                    this.onChildrenMounted()
                }
                if (!objectInfo.hasBroadcastDescendantsMounted) {
                    objectInfo.hasBroadcastDescendantsMounted = true
                    window.$components.objectRegistry.set(this.id, objectInfo)
                    this.onDescendantsMounted()
                }
                return
            }

            const truncatedMessageId = message.id.replace(this.id, ``)
            const rebuiltMessageId = this.id + truncatedMessageId

            if (objectInfo.mountedChildComponents.includes(message.id)) { return }
            if (rebuiltMessageId !== message.id) { return }

            if (childComponents.includes(truncatedMessageId) && !objectInfo.hasBroadcastChildrenMounted) {
                objectInfo.mountedChildComponents.push(message.id)
                if (childComponents.length === objectInfo.mountedChildComponents.length) {
                    objectInfo.hasBroadcastChildrenMounted = true
                    window.$components.objectRegistry.set(this.id, objectInfo)
                    this.onChildrenMounted()
                }
            }

            if (this.haveDescendantsMounted() && !objectInfo.hasBroadcastDescendantsMounted) {
                objectInfo.hasBroadcastDescendantsMounted = true
                window.$components.objectRegistry.set(this.id, objectInfo)
                this.onDescendantsMounted()
            }
        })
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_INITIALIZATION, this)
    }
    haveChildrenMounted() {
        const childComponents = window.$components.childComponentRegistry.get(this.className())
        const objectInfo = window.$components.objectRegistry.get(this.id)

        if (!childComponents || 0 === childComponents.length) { return true }
        return childComponents.length === objectInfo.mountedChildComponents.length
    }
    haveDescendantsMounted() {
        if (!this.haveChildrenMounted()) { return false }

        const childComponents = window.$components.childComponentRegistry.get(this.className())

        if (!childComponents || 0 === childComponents.length) { return true }

        for (let childComponentId of childComponents) {
            if (!Component.isObjectRegistered(`${this.id}${childComponentId}`)) { return false }
            if (!Component.getObject(`${this.id}${childComponentId}`).haveDescendantsMounted()) { 
                return false 
            }
        }
        return true
    }
    mount() { ComponentLifecycle.mount(this.id) }
    beforeMount() { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_MOUNT, this )}
    afterMount() { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_MOUNT, this )}
    unmount() { ComponentLifecycle.unmount(this.id) }
    beforeUnmount() { 
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_UNMOUNT, this )

        const childComponents = window.$components.childComponentRegistry.get(this.className())

        if (!childComponents || 0 === childComponents.length) { return }

        for (let childComponentId of childComponents) {
            if (!Component.isObjectRegistered(`${this.id}${childComponentId}`)) { continue }
            
            const child = Component.getObject(`${this.id}${childComponentId}`)
            
            child.unmount()
        }
    }
    afterUnmount() { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_UNMOUNT, this )}
    isMounted() {
        if (!window.$components?.objectRegistry?.has(this.id)) { return false }
        return window.$components.objectRegistry.get(this.id).mounted
    }
    onChildrenMounted() {
        Loader.addChildComponentGettersToComponentObject(this.className(), this.id)
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_CHILDREN_MOUNTED, this )
    }
    onDescendantsMounted() { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_DESCENDANTS_MOUNTED, this )}
    isMounted() { return window.$components.objectRegistry.get(this.id).mounted } 
    destroy() { 
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_DESTRUCTION, this)

        const childComponents = window.$components.childComponentRegistry.get(this.className())

        if (childComponents && 0 !== childComponents.length) {
            for (let childComponentId of childComponents) {
                if (!Component.isObjectRegistered(`${this.id}${childComponentId}`)) { continue }
                
                const child = Component.getObject(`${this.id}${childComponentId}`)
                
                child.destroy()
            }
        }
        if (window.$components?.objectRegistry?.has(this.id)) { ComponentLifecycle.destroyComponentObject(`${this.id}`) }
        Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_DESTRUCTION, this)
    }
    get Parent() {
        let element = document.getElementById(this.id)
        let walkUpTree = (element) => {
            while (element.parentElement) {
                if (element.parentElement.id) {
                    let parentComponent = window?.$components?.objectRegistry?.get(element.parentElement.id)
    
                    if (parentComponent) { 
                        return parentComponent.componentObject
                    } 
                }
                element = element.parentElement
            }
            return null
        }

        if (!element) { return null }
        return walkUpTree(element)
    }
}