class Tree {
    constructor() {
        this.nodes = []
    }
    addNode(node) {
        if (this.hasNode(node.name)) { return }
        this.nodes.push(node)
    }
    hasNode(name) {
        let node = this.getNodeByName(name)
        return (null !== node) 
    }
    getNodeByName(name) {
        for (let node of this.nodes) {
            if (node.name === name) { return node }
            let descendant = node.getDescendant(name)
            if (descendant) { return descendant }
        }
        return null
    }
    toString(prefix) {
        let result = ``
        for (let node of this.nodes) {
            result += node.toString(prefix)
        }
        return result
    }
}