class SlottedComponent extends Component {
    static createSlotMarkup = (componentId, slotId, parentElement) => {
        let newSlotMarkup = document.createElement(`slot-markup`)

        newSlotMarkup.setAttribute(`for-component-id`, componentId)
        newSlotMarkup.setAttribute(`for-slot`, slotId)
        parentElement.appendChild(newSlotMarkup)
        return newSlotMarkup
    }
    static loadSlotFromInclude = async (parentElement, slotId, includeIn, src, componentClass, componentId, props, vars) => {
        const newSlotContent = SlottedComponent.createSlotMarkup(parentElement.id, slotId, parentElement)
        const newInclude = Component.createComponentInclude(includeIn, src, componentClass, componentId, props, vars)

        newSlotContent.appendChild(newInclude)
        await Loader.loadInclude(newInclude)
        
        SlotManager.loadSlot(newSlotContent) 
        Loader.loadIncludes()
    }
    beforeSlotLoaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_SLOT_LOADED, { component: this, slot })}
    afterSlotLoaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_SLOT_LOADED, { component: this, slot } )}
    beforeSlotUnloaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_SLOT_UNLOADED, { component: this, slot })}
    afterSlotUnloaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_SLOT_UNLOADED, { component: this, slot } )}
    isSlotted(slotId) { return SlotManager.isSlotted(this.id, slotId) }
    slot(element, slotId) { SlotManager.moveSlotContentToComponent(this, element, document.getElementById(slotId)) }
    unslot(slotId) { SlotManager.unslot(slotId) }
}