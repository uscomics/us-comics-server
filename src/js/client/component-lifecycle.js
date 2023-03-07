/*
    FRAGRMENT STATE             FRAGMENT STATE                      NOTES
    (Transitions flow down)     (Transitions flow left/right)
    -------------------------   ----------------------------------  ---------------------------------------------------------
    Compile                                                         Creates document fragment from text.
    Register Dom Fragment       <--> Unregister Dom Fragment        Document fragment put in registry, script tags moved to <HEAD>.

    COMPONENT STATE             COMPONENT STATE                     NOTES
    (Transitions flow down)     (Transitions flow left/right)
    -------------------------   ----------------------------------  ---------------------------------------------------------
    Create Component Object                                         Object instantiated from the class representing the component.
                                                                    Object initialized. Vars and props are wrapped. Node values and
                                                                    attribute values are replaced for the first time.
    Register Component Object   <--> Unregister Component Object    Object placed in registery. Has a unique ID and knows the id
                                                                    of its associated fragment.
    mount                       <--> unmount                        Component placed in DOM, rendered to screen.
                                before load slot                    Slots can be loaded once the parent containing the slot is mounted.
                                after load slot
                                before unload slot                  Slots can be unloaded after they have been loaded.
                                after unload slot
    children mounted                                                The child components of a component have mounted.
    descendants mounted                                             All descendant components of a component have mounted.
    update                                                          Not truely a lifecycle state. This is when you'll set vars to replace
                                                                    node and attribute values.
    destroy                                                         The object is unmounted, unregistereed, and it's marker is removed
                                                                    from the DOM.
*/
class ComponentLifecycle {
    static msgs = {
        COMPONENT_BEFORE_INITIALIZATION:    `COMPONENT_BEFORE_INITIALIZATION`,
        COMPONENT_AFTER_INITIALIZATION:     `COMPONENT_AFTER_INITIALIZATION`,
        COMPONENT_BEFORE_MOUNT:             `COMPONENT_BEFORE_MOUNT`,
        COMPONENT_AFTER_MOUNT:              `COMPONENT_AFTER_MOUNT`,
        COMPONENT_BEFORE_UNMOUNT:           `COMPONENT_BEFORE_UNMOUNT`,
        COMPONENT_AFTER_UNMOUNT:            `COMPONENT_AFTER_UNMOUNT`,
        COMPONENT_CHILDREN_MOUNTED:         `COMPONENT_CHILDREN_MOUNTED`,
        COMPONENT_DESCENDANTS_MOUNTED:      `COMPONENT_DESCENDANTS_MOUNTED`,
        COMPONENT_BEFORE_DESTRUCTION:       `COMPONENT_BEFORE_DESTRUCTION`,
        COMPONENT_AFTER_DESTRUCTION:        `COMPONENT_AFTER_DESTRUCTION`,
        COMPONENT_BEFORE_SLOT_LOADED:       `COMPONENT_BEFORE_SLOT_LOADED`,
        COMPONENT_AFTER_SLOT_LOADED:        `COMPONENT_AFTER_SLOT_LOADED`,
        COMPONENT_BEFORE_SLOT_UNLOADED:     `COMPONENT_BEFORE_SLOT_UNLOADED`,
        COMPONENT_AFTER_SLOT_UNLOADED:      `COMPONENT_AFTER_SLOT_UNLOADED`
        }
    static initialize() {
        window.$components = undefined
    }
    static saveOriginalNodeValues = (node) => {
        if (node.nodeValue) {
            if (!node.originalNodeValue) { node.originalNodeValue = node.nodeValue }
        }
        for (let child of node.childNodes) {
            ComponentLifecycle.saveOriginalNodeValues(child)
        }
    }
    static saveOriginalNodeAttributes = (node) => {
        if (node.attributes) {
            for (const attr of node.attributes) {
                if (!attr.originalAttributeValue) { attr.originalAttributeValue = attr.value }
            }
        }
        for (let child of node.childNodes) {
            ComponentLifecycle.saveOriginalNodeAttributes(child)
        }
    }
    static copyOriginalNodeValues = (srcNode, destNode) => {
        if (srcNode.nodeValue) {
            if (srcNode.originalNodeValue && !destNode.originalNodeValue) { 
                destNode.originalNodeValue = srcNode.originalNodeValue
            }
        }
        for (let loop = 0; loop < srcNode.childNodes.length; loop++) {
            let srcChildNode = srcNode.childNodes[loop]
            let destChildNode = destNode.childNodes[loop]
            ComponentLifecycle.copyOriginalNodeValues(srcChildNode, destChildNode)
        }
    }
    static copyOriginalNodeAttributes = (srcNode, destNode) => {
        if (srcNode.attributes) {
            for (let loop = 0; loop < srcNode.attributes.length; loop++) {
                let srcAttribute = srcNode.attributes[loop]
                let destAttribute = destNode.attributes[loop]

                if (srcAttribute.originalAttributeValue && !destAttribute.originalAttributeValue) { 
                    destAttribute.originalAttributeValue = srcAttribute.originalAttributeValue 
                }
            }
        }
        for (let loop = 0; loop < srcNode.childNodes.length; loop++) {
            let srcChildNode = srcNode.childNodes[loop]
            let destChildNode = destNode.childNodes[loop]
            ComponentLifecycle.copyOriginalNodeAttributes(srcChildNode, destChildNode)
        }
    }
    static replaceNodeValue = (node, data, member) => {
        if (`CalendarTitle` === node.id) {
            const x = 1
        }
        if (node.nodeValue) {
            if (!node.originalNodeValue) { node.originalNodeValue = node.nodeValue }
            if (!node.replacements) { node.replacements = new Map() }

            const formattedMember = `{${member}}`
            const originalMatches = -1 !== node.originalNodeValue.indexOf(formattedMember)
            const matches = -1 !== node.nodeValue.indexOf(formattedMember)
            if (originalMatches || matches) {
                // TODO: Thoroughly test this with multiple replacements in a single node value.
                let nodeValue = (matches)? node.nodeValue : node.originalNodeValue

                try {
                    const memberData = data[member].toString()

                    node.replacements.set(member, memberData)
                    for (let [key, value] of node.replacements) {
                        const curlyBracesRegEx = new RegExp('{' + key + '}', 'g')

                        nodeValue = nodeValue.replace(curlyBracesRegEx, value)
                    }
                    node.nodeValue = nodeValue
                } catch (e) {
                    console.warn(`Unable to replace node value containing ${member}. It's value is ${nodeValue}. It's replacement value is ${data[member]}. Node id is ${node.id}`)
                }
            }
        }
        for (let child of node.childNodes) {
            ComponentLifecycle.replaceNodeValue(child, data, member)
        }
    }
    static replaceAttributeValue = (node, data, member) => {
        if (node.attributes) {
            for (const attr of node.attributes) {
                if (!attr.originalAttributeValue) { attr.originalAttributeValue = attr.value }
                if (!attr.replacements) { attr.replacements = new Map() }

                const formattedMember = `{${member}}`
                const originalMatches = -1 !== attr.originalAttributeValue.indexOf(formattedMember)
                const matches = -1 !== attr.value.indexOf(formattedMember)
                let attrValue = (matches)? attr.value : attr.originalAttributeValue

                if (originalMatches || matches) {
                    // TODO: Thoroughly test this with multiple replacements in a single attribute value.
                    try {
                        let memberData = data[member].toString()
                    
                        attr.replacements.set(member, memberData)
                        for (let [key, value] of attr.replacements) {
                            const curlyBracesRegEx = new RegExp('{' + key + '}', 'g')
    
                            attrValue = attrValue.replace(curlyBracesRegEx, value)
                        }
                        attr.value = attrValue
                    } catch (e) {
                        console.warn(`Unable to replace attribute containing ${member}. It's replacement value is ${data[member]}. Node id is ${node.id}`)
                    }
                }
            }
        }
        for (let child of node.childNodes) {
            ComponentLifecycle.replaceAttributeValue(child, data, member)
        }
    }
    static wrapProps = (componentFragment, componentObject) => {
        if (!componentObject.props) {return}
        let members = Object.getOwnPropertyNames(componentObject.props)

        componentObject.props.$propsStore = {...componentObject.props}
        for (let member of members) {
            if (`$propsStore` === member) { continue }
            Object.defineProperty(componentObject.props, member, {
                get: function() {
                    return componentObject.props.$propsStore[member]
                },
                set: function(newValue) {
                    console.error(`wrapProps: Cannot set ${member}.`)
                }
            })
            ComponentLifecycle.replaceNodeValue(componentFragment, componentObject.props, member)
            ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject.props, member)
        }
    }
    static wrapVars = (componentFragment, componentObject) => {
        if (!componentObject.vars) {return}
        let members = Object.getOwnPropertyNames(componentObject.vars)

        componentObject.vars.$varsStore = {...componentObject.vars}
        for (let member of members) {
            if (`$varsStore` === member) { continue }
            Object.defineProperty(componentObject.vars, member, {
                get: function() {
                    return componentObject.vars.$varsStore[member]
                },
                set: function(newValue) {
                    const oldValue = componentObject.vars.$varsStore[member]

                    componentObject.vars.$varsStore[member] = newValue
                    ComponentLifecycle.replaceNodeValue(componentFragment, componentObject.vars, member)
                    ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject.vars, member)
                    Queue.broadcast(Messages.VALUE_CHANGED, { componentObject, member, oldValue, newValue})
                }
            })
            ComponentLifecycle.replaceNodeValue(componentFragment, componentObject.vars, member)
            ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject.vars, member)
        }
    }
    static compile = (html) => {
        let fragment = document.createDocumentFragment()

        fragment.append(...new DOMParser().parseFromString(html, `text/html`).body.childNodes)
        return fragment
    }
    static registerDOMFragment = (componentClass, componentFragment, includeTest) => {
        if (!componentClass) { 
            console.error(`registerDOMFragment: No component class provided for DOM fragment registration.`)
            return false 
        }
        if (!componentFragment) { 
            console.error(`registerDOMFragment: No DOM fragment provided for DOM fragment registration.`)
            return false 
        }
        if (window?.$components?.fragmentRegistry?.has(componentClass)) { 
            console.info(`registerDOMFragment: DOM Fragment ${componentClass} is already registered.`)
            return true 
        }

        let scripts = componentFragment.querySelectorAll(`script`)
        let tests = componentFragment.querySelectorAll(`test-script`)
        let styles = componentFragment.querySelectorAll(`style`)
        let markup = componentFragment.querySelectorAll(`component-markup`)

        if (0 === scripts.length) {
            console.error(`registerDOMFragment: Fragment must contain at least one component script tag.`)
            return false
        }
        if (1 !== markup.length) {
            console.error(`registerDOMFragment: Fragment must contain one and only one component markup tag.`)
            return false
        }
        if (1 < styles.length) {
            console.error(`registerDOMFragment: Fragment can contain no more than one component style tag.`)
            return false
        }
        if (includeTest) {
            if (1 < tests.length) {
                console.error(`registerDOMFragment: Fragment can contain no more than one component test tag.`)
                return false
            }   
        }
        let scriptTag = document.createElement(`script`)

        scriptTag.type = `text/javascript`
        scriptTag.id = `ScriptTag${componentClass}`
        scripts[0].remove()
        try {
            eval(`new ${componentClass}`)
            scriptTag.appendChild(document.createTextNode(``))
        } catch (e) {
            scriptTag.appendChild(document.createTextNode(scripts[0].innerText))
        }
        document.head.appendChild(scriptTag);

        if (styles.length && !document.getElementById(`StyleTag${componentClass}`)) {
            let styleTag = document.createElement(`style`)

            styleTag.id = `StyleTag${componentClass}`
            styleTag.appendChild(document.createTextNode(styles[0].innerText))
            styles[0].remove()
            document.head.appendChild(styleTag);
        }
        if (tests.length) {
            if (includeTest) {
                let tests = componentFragment.querySelectorAll(`test-script`)
                let testTag = document.createElement(`test-script`)

                testTag.type = 'text/javascript'
                testTag.id = `TestTag${componentClass}`
                testTag.appendChild(document.createTextNode(tests[0].innerText))
                document.head.appendChild(testTag);
            }
            tests[0].remove()
        }
        if (!window.$components) { window.$components = {} }
        if (!window.$components.fragmentRegistry) { window.$components.fragmentRegistry = new Map() }
        ComponentLifecycle.saveOriginalNodeValues(componentFragment)
        ComponentLifecycle.saveOriginalNodeAttributes(componentFragment)
        window.$components.fragmentRegistry.set(componentClass, componentFragment)

        return true
    }
    static unregisterDOMFragment = (componentClass) => {
        if (!componentClass) { 
            console.error(`unregisterDOMFragment: No component class provided for unregistration.`)
            return false 
        }
        if (!window?.$components?.fragmentRegistry?.has(componentClass)) { 
            console.error(`unregisterDOMFragment: DOM Fragment ${componentClass} was not in registery.`)
            return false 
        }
        let componentScriptTag = document.getElementById(`ScriptTag${componentClass}`)
        let componentStyleTag = document.getElementById(`StyleTag${componentClass}`)
        let componentTestTag = document.getElementById(`TestTag${componentClass}`)
        
        if (componentScriptTag) { componentScriptTag.remove() }
        if (componentStyleTag) { componentStyleTag.remove() }
        if (componentTestTag) { componentTestTag.remove() }
        window.$components.fragmentRegistry.delete(componentClass)
        return true
    }
    static createComponentObject = (componentClass, componentObjectId, includeElement) => {

        if (!componentClass) { 
            console.error(`createComponentObject: No component class provided for createComponentObject.`)
            return false 
        }
        if (!componentObjectId) { 
            console.error(`createComponentObject: No component object id provided for createComponentObject.`)
            return false 
        }
        if (!includeElement) { 
            console.error(`createComponentObject: No includeComponentElement provided for createComponentObject.`)
            return false 
        }

        let componentObject = eval(`new ${componentClass}()`)
        let markerId = `-ComponentBeginMarker${componentObjectId}`
        let marker = document.getElementById(markerId)
        let componentFragment =  window.$components.fragmentRegistry.get(componentClass)

        if (!marker) {
            let marker = document.createElement(`script`)

            marker.id = markerId
            marker.type = `text/javascript`
            // TODO: Do not replace child until repeat is done.
            includeElement.parentNode.replaceChild(marker, includeElement);
        }

        let propsElements = includeElement.getElementsByTagName(`include-props`)
        let varsElements = includeElement.getElementsByTagName(`include-vars`)

        if (propsElements.length) {
            if (1 < propsElements.length) { 
                console.error(`createComponentObject: Only one include-props tag is allowed.`)
                return false 
            }
            
            let jsonText = `(` + DOMPurify.sanitize(propsElements[0].innerText) + `)`
            let propsObject = eval(jsonText)

            componentObject.props = {...componentObject.props, ...propsObject}
        }
        if (varsElements.length) {
            if (1 < varsElements.length) { 
                console.error(`createComponentObject: Only one include-vars tag is allowed.`)
                return false 
            }

            let jsonText = `(` + DOMPurify.sanitize(varsElements[0].innerText) + `)`
            let varsObject = eval(jsonText)

            componentObject.vars = {...componentObject.vars, ...varsObject}
        }
        if (componentObject.initialize) { componentObject.initialize(componentObjectId) }

        ComponentLifecycle.replaceNodeValue(componentFragment, componentObject, `id`)
        ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject, `id`)

        if (componentObject.props) {
            let members = Object.getOwnPropertyNames(componentObject.props)

            for (let member of members) {
                ComponentLifecycle.replaceNodeValue(componentFragment, componentObject.props, member)
                ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject.props, member)
            }
        }

        if (componentObject.vars) {
            let members = Object.getOwnPropertyNames(componentObject.vars)

            for (let member of members) {
                ComponentLifecycle.replaceNodeValue(componentFragment, componentObject.vars, member)
                ComponentLifecycle.replaceAttributeValue(componentFragment, componentObject.vars, member)
            }
        }
        return componentObject
    }
    static registerComponentObject = (componentClass, componentObjectId, componentObject, includeElement) => {
        if (!componentObjectId) { 
            console.error(`registerComponentObject: No component object id provided for component object registration.`)
            return false 
        }
        if (!componentObject) { 
            console.error(`registerComponentObject: No component object provided for component object registration.`)
            return false 
        }
        if (!componentClass) { 
            console.error(`registerComponentObject: No fragment id provided for component object registration.`)
            return false 
        }
        if (!includeElement) { 
            console.error(`registerComponentObject: No include element provided for component object registration.`)
            return false 
        }
        if (window?.$components?.objectRegistry?.has(componentObjectId)) { 
            console.error(`registerComponentObject: Component object ${componentObjectId} is already registered.`)
            return false 
        }

        let componentDOM = []
        let fragment = window.$components.fragmentRegistry.get(componentClass)
        let markup = fragment.querySelector(`component-markup`)
        let clonedFragment = fragment.cloneNode(true)
        let clonedMarkup = clonedFragment.querySelector(`component-markup`)

        if (!clonedMarkup) { 
            console.error(`registerComponentObject: Markup for ${componentObjectId} not found.`)
            return false 
        }

        if (!window.$components) { window.$components = {} }
        if (!window.$components.objectRegistry) { window.$components.objectRegistry = new Map() }
        for (let loop = clonedMarkup.children.length - 1; loop >= 0; loop--) {
            let originalChild = markup.children[loop]
            let clonedChild = clonedMarkup.children[loop]
            const addElementGettersToComponentObject = (element, componentObject) => {
                for (let child of element.children) {
                    addElementGettersToComponentObject(child, componentObject)
                }
                if (!element.id || -1 !== element.tagName.indexOf(`-`)) { return }
                let getterName = element.id.replace(` `, `_`).replace(componentObject.id, ``)
    
                getterName += `Element`
                if (!componentObject.hasOwnProperty(getterName)) {
                    Object.defineProperty(componentObject, getterName, {
                        get: function() {
                            return document.getElementById(element.id)
                        },
                        set: function(newValue) {
                            console.error(`wrapProps: Cannot set ${getterName}.`)
                        }
                    })
                }
            }
            const addConvenienceMethodsToElement = (element) => {
                const show = () => { if (element.classList.contains(`display-none`)) { element.classList.remove(`display-none`) }}
                const hide = () => { if (!element.classList.contains(`display-none`)) { element.classList.add(`display-none`) }}
                const isVisible = () => { return !element.classList.contains(`display-none`) }
                const toggleVisibility = () => { if (element.isVisible()) { element.hide() } else {element.show() }}
                const removeChildren = () => { while (element.firstChild) { element.removeChild(element.firstChild) }}

                element.show = show
                element.hide = hide
                element.isVisible = isVisible
                element.toggleVisibility = toggleVisibility
                element.removeChildren = removeChildren
                for (const elementChild of element.children) {
                    addConvenienceMethodsToElement(elementChild)
                }
            }
            const setEventHandler = (node, event) => {
                let eventHandlerText = node.getAttribute(event)

                if (eventHandlerText && -1 !== eventHandlerText.indexOf(`$obj.`)) {
                    eventHandlerText = eventHandlerText.replaceAll(`$obj.`, `Component.getObject('${componentObjectId}').`)
                    node.setAttribute(event, eventHandlerText)
                }
                for (const nodeChild of node.children) {
                    setEventHandler(nodeChild, event)
                }
            }
            const copyAttributes = (includeElementSrc, clonedChildDest) => {
                for (let attributeLoop = 0; attributeLoop < includeElementSrc.attributes.length; attributeLoop++) {
                    let attribute = includeElementSrc.attributes[attributeLoop]
                    if (`include-in` === attribute.name) { continue }
                    if (`src` === attribute.name) { continue }
                    if (`component-class` === attribute.name) { continue }
                    if (`component-id` === attribute.name) { continue }
                    if (`repeat` === attribute.name) { continue }

                    let attributeValue = ``

                    if (clonedChildDest.hasAttribute(attribute.name)) { attributeValue = clonedChildDest.getAttribute(attribute.name) }
                    clonedChildDest.setAttribute(attribute.name, attributeValue + attribute.value)
                }
            }
            if (0 === loop) { copyAttributes(includeElement, clonedChild) }

            componentDOM.push(clonedChild)
            setEventHandler(clonedChild, `onblur`)
            setEventHandler(clonedChild, `onchange`)
            setEventHandler(clonedChild, `oncontextmenu`)
            setEventHandler(clonedChild, `onfocus`)
            setEventHandler(clonedChild, `oninput`)
            setEventHandler(clonedChild, `oninvalid`)
            setEventHandler(clonedChild, `onreset`)
            setEventHandler(clonedChild, `onsearch`)
            setEventHandler(clonedChild, `onselect`)
            setEventHandler(clonedChild, `onsubmit`)
            setEventHandler(clonedChild, `onkeydown`)
            setEventHandler(clonedChild, `onkeyup`)
            setEventHandler(clonedChild, `onclick`)
            setEventHandler(clonedChild, `ondblclick`)
            setEventHandler(clonedChild, `onmousedown`)
            setEventHandler(clonedChild, `onmousemove`)
            setEventHandler(clonedChild, `onmouseout`)
            setEventHandler(clonedChild, `onmouseover`)
            setEventHandler(clonedChild, `onmouseup`)
            setEventHandler(clonedChild, `onwheel`)
            setEventHandler(clonedChild, `ondrag`)
            setEventHandler(clonedChild, `ondragend`)
            setEventHandler(clonedChild, `ondragenter`)
            setEventHandler(clonedChild, `ondragleave`)
            setEventHandler(clonedChild, `ondragover`)
            setEventHandler(clonedChild, `ondragstart`)
            setEventHandler(clonedChild, `ondrop`)
            setEventHandler(clonedChild, `onscroll`)
            setEventHandler(clonedChild, `oncopy`)
            setEventHandler(clonedChild, `oncut`)
            setEventHandler(clonedChild, `onpaste`)
            setEventHandler(clonedChild, `onabort`)
            setEventHandler(clonedChild, `oncanplay`)
            setEventHandler(clonedChild, `oncanplaythrough`)
            setEventHandler(clonedChild, `oncuechange`)
            setEventHandler(clonedChild, `ondurationchange`)
            setEventHandler(clonedChild, `onemptied`)
            setEventHandler(clonedChild, `onended`)
            setEventHandler(clonedChild, `onerror`)
            setEventHandler(clonedChild, `onloadeddata`)
            setEventHandler(clonedChild, `onloadedmetadata`)
            setEventHandler(clonedChild, `onloadstart`)
            setEventHandler(clonedChild, `onpause`)
            setEventHandler(clonedChild, `onplay`)
            setEventHandler(clonedChild, `onplaying`)
            setEventHandler(clonedChild, `onprogress`)
            setEventHandler(clonedChild, `onratechange`)
            setEventHandler(clonedChild, `onseeked`)
            setEventHandler(clonedChild, `onseeking`)
            setEventHandler(clonedChild, `onstalled`)
            setEventHandler(clonedChild, `onsuspend`)
            setEventHandler(clonedChild, `ontimeupdate`)
            setEventHandler(clonedChild, `onvolumechange`)
            setEventHandler(clonedChild, `onwaiting`)
            setEventHandler(clonedChild, `ontoggle`)
            addElementGettersToComponentObject(clonedChild, componentObject)
            addConvenienceMethodsToElement(clonedChild)
            ComponentLifecycle.copyOriginalNodeValues(originalChild, clonedChild)
            ComponentLifecycle.copyOriginalNodeAttributes(originalChild, clonedChild)
        }

        for (let node of componentDOM) {
            ComponentLifecycle.wrapVars(node, componentObject)
            ComponentLifecycle.wrapProps(node, componentObject)
            ComponentLifecycle.replaceNodeValue(node, componentObject, `id`)
            ComponentLifecycle.replaceAttributeValue(node, componentObject, `id`)
        }

        window.$components.objectRegistry.set(componentObjectId, { componentObject, componentClass, componentDOM, mounted: false, mountedChildComponents: [], hasBroadcastChildrenMounted: false, hasBroadcastDescendantsMounted: false })
        return true
    }
    static unregisterComponentObject = (componentObjectID) => {
        if (!componentObjectID) { 
            console.error(`unregisterComponentObject: No component object id provided for registration.`)
            return false 
        }
        if (!window?.$components?.objectRegistry?.has(componentObjectID)) { 
            console.error(`unregisterComponentObject: Component object ${componentObjectID} was not in registery.`)
            return false 
        }
        if (window.$components.objectRegistry.get(componentObjectID).mounted) { 
            console.error(`unregisterComponentObject: Cannot unregister a mounted component, ${componentObjectID}.`)
            return false 
        }
        window.$components.objectRegistry.delete(componentObjectID)
        return true
    }
    static mount = (componentObjectId) => {
        if (!componentObjectId) { 
            console.error(`unregisterComponentObject: No component object id provided for mount.`)
            return false 
        }
        if (!window?.$components?.objectRegistry?.has(componentObjectId)) { 
            console.error(`unregisterComponentObject: Component object ${componentObjectId} was not in registery.`)
            return false 
        }

        let componentObjectInfo = window.$components.objectRegistry.get(componentObjectId)
        let fragment = window.$components.fragmentRegistry.get(componentObjectInfo.componentClass)
        let beginMarkerId = `-ComponentBeginMarker${componentObjectId}`
        let beginMarker = document.getElementById(beginMarkerId)

        if (!fragment) { 
            console.error(`unregisterComponentObject: DOM fragment ${componentObjectInfo.componentClass} is not in registery.`)
            return false 
        }
        if (!componentObjectInfo.componentObject) { 
            console.error(`unregisterComponentObject: Component object ${componentObjectId} is not in registery.`)
            return false 
        }
        if (componentObjectInfo.mounted) { 
            console.error(`unregisterComponentObject: Component object ${componentObjectId} is already mounted.`)
            return false 
        }
        if (!beginMarker) { 
            console.error(`UnregisterComponentObject: Marker for ${componentObjectId} is not in DOM.`)
            return false 
        }
        if (componentObjectInfo.componentObject.beforeMount) { componentObjectInfo.componentObject.beforeMount() }

        let endMarkerId = `-ComponentEndMarker${componentObjectId}`
        let endMarker = document.createElement(`script`)

        endMarker.id = endMarkerId
        beginMarker.after(endMarker)
        for (let child of componentObjectInfo.componentDOM) {
            beginMarker.after(child)
        }

        componentObjectInfo.mounted = true
        window.$components.objectRegistry.set(componentObjectId, componentObjectInfo)
        if (componentObjectInfo.componentObject.afterMount) { componentObjectInfo.componentObject.afterMount() }
        return true
    }
    static unmount = (componentObjectId) => {
        if (!componentObjectId) { 
            console.error(`Unmount: No component object id provided for mount.`)
            return false 
        }
        if (!window?.$components?.objectRegistry?.has(componentObjectId)) { 
            console.error(`Unmount: Component object ${componentObjectId} is not in registery.`)
            return false 
        }

        let componentObjectInfo = window?.$components?.objectRegistry?.get(componentObjectId)

        if (!componentObjectInfo?.componentObject) { 
            console.error(`Unmount: Component object ${componentObjectId} is not in registery.`)
            return false 
        }
        if (!componentObjectInfo.mounted) { 
            console.error(`Unmount: Component object ${componentObjectId} was not mounted.`)
            return false 
        }

        let fragment = window.$components.fragmentRegistry.get(componentObjectInfo.componentClass)
        let beginMarkerId = `-ComponentBeginMarker${componentObjectId}`
        let beginMarker = document.getElementById(beginMarkerId)
        let endMarkerId = `-ComponentEndMarker${componentObjectId}`
        let endMarker = document.getElementById(endMarkerId)
        let markup = fragment.querySelector(`component-markup`)

        if (!fragment) { 
            console.error(`Unmount: Fragment ${componentObjectInfo.componentClass} is not in registery.`)
            return false 
        }
        if (!beginMarker) { 
            console.error(`Unmount: Begin Marker for ${componentObjectId}, ${beginMarkerId}, not in DOM.`)
            return false 
        }
        if (!endMarker) { 
            console.error(`Unmount: End Marker for ${componentObjectId}, ${beginMarkerId}, not in DOM.`)
            return false 
        }
        if (!markup) { 
            console.error(`Unmount: Markup for ${componentObjectId} not found.`)
            return false 
        }

        if (componentObjectInfo.componentObject.beforeMount) { componentObjectInfo.componentObject.beforeUnmount() } 
        while (beginMarker.nextSibling && beginMarker.nextSibling.id !== endMarkerId) {
            beginMarker.nextSibling.remove()
        }
        endMarker.remove()
        
        componentObjectInfo.mounted = false
        window.$components.objectRegistry.set(componentObjectId, componentObjectInfo)
        if (componentObjectInfo.componentObject.afterMount) { componentObjectInfo.componentObject.afterUnmount() }
        return true
    }
    static destroyComponentObject(componentObjectId) {
        let beginMarkerId = `-ComponentBeginMarker${componentObjectId}`
        let endMarkerId = `-ComponentEndMarker${componentObjectId}`
        let beginMarker = document.getElementById(beginMarkerId)
        let endMarker = document.getElementById(endMarkerId)
        let component = Component.getObject(componentObjectId)

        if (component && component.isMounted()) { ComponentLifecycle.unmount(componentObjectId) }
        ComponentLifecycle.unregisterComponentObject(componentObjectId)
        if (beginMarker) { beginMarker.remove() }
        if (endMarker) { endMarker.remove() }
    }
}