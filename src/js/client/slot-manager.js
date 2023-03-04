class SlotManager {
    static loadSlots() {
        const getSlot = (forComponentId, slotName) => {
            const componentElement = document.getElementById(forComponentId)
    
            if (!componentElement) {
                console.error(`getSlot: Component element ${forComponentId} was not found.`)
                return null
            }
    
            const componentSlot = componentElement.querySelector(`component-slot[id=${slotName}]`)
    
            if (!componentSlot) {
                console.error(`getSlot: Component slot ${slotName} was not found.`)
                return null
            }
    
            return componentSlot
        }
        const moveSlotContentToComponent = (component, slotContentElement, componentSlotElement) => {
            const addElementGettersToComponentObject = (element, componentObject) => {
                if (element.children) {
                    for (let child of element.children) {
                        addElementGettersToComponentObject(child, componentObject)
                    }
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
            }

            if (0 === slotContentElement.children.length) {
                console.error(`moveSlotContentToComponent: Slot content element has no children.`)
                return false
            }
            if (0 !== componentSlotElement.children.length) {
                console.error(`moveSlotContentToComponent: Component slot ${componentSlotElement.getAttribute(`for-slot`)} was not found.`)
                return false
            }
    
            while (0 < slotContentElement.children.length) {
                componentSlotElement.after(slotContentElement.lastChild)
                addElementGettersToComponentObject(slotContentElement.lastChild, component)
                addConvenienceMethodsToElement(slotContentElement.lastChild)
            }
            componentSlotElement.remove()
            slotContentElement.remove()
    
            return true
        }

        let slotMarkupElements = document.querySelectorAll('slot-markup')

        for (let slotContent of slotMarkupElements) {
            const forComponentId = slotContent.getAttribute(`for-component-id`)

            if (!forComponentId) {
                console.error(`loadSlots: The for-component-id attribute is required on slot-markup tags.`)
                continue
            }

            const slotName = slotContent.getAttribute(`for-slot`)
            const componentSlot = getSlot(forComponentId, slotName)
            const component = Component.getObject(forComponentId)

            if (!componentSlot) { continue }
            component?.beforeSlotLoaded(slotName)
            if (!moveSlotContentToComponent(component, slotContent, componentSlot)) {
                console.error(`loadSlots: An error occured while moving slot content to the ${componentElement.getAttribute(`for-slot`)}] slot of ${forComponentId}.`)
                continue
            }
            component?.afterSlotLoaded(slotName)
        }
    }
}