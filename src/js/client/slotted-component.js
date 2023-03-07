class SlottedComponent extends Component {
    beforeSlotLoaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_SLOT_LOADED, { component: this, slot })}
    afterSlotLoaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_SLOT_LOADED, { component: this, slot } )}
    beforeSlotUnloaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_BEFORE_SLOT_UNLOADED, { component: this, slot })}
    afterSlotUnloaded(slot) { Queue.broadcast(ComponentLifecycle.msgs.COMPONENT_AFTER_SLOT_UNLOADED, { component: this, slot } )}
    slot(element, slotId) { SlotManager.moveSlotContentToComponent(this, element, document.getElementById(slotId)) }
    unslot(slotId) { SlotManager.removeSlotContentFromComponent(slotId) }
}