class TreeNode {
    constructor(name) {
        this.name = name
        this.children = []
        this.parent = null
    }
    addChild(child) {
        let childNode = new TreeNode(child)
        childNode.parent = this
        this.children.push(childNode)
        return childNode
    }
    hasChild(name) {
        let node = this.getChildByName(name)
        return (null !== node) 
    }
    getChildByName(name) {
        for (let node of this.children) {
            if (node.name === name) { return node }
        }
        return null
    }
    hasAncestor(ancestorName) {
        let node = this.getAncestor(ancestorName)
        return (null !== node)
    }
    getAncestor(ancestorName) {
        if (!this.parent) {return null}
        if (this.parent.name === ancestorName) {return this.parent}
        return this.parent.getAncestor(ancestorName)
    }
    hasDescendant(descendantName) {
        let node = this.getDescendant(descendantName)
        return (null !== node)
    }
    getDescendant(descendantName) {
        for (let node of this.children) {
            if (node.name === descendantName) { return node }
            let descendant = node.getDescendant(descendantName)
            if (descendant) { return descendant }
        }
        return null
    }
    toString(prefix) {
        let cleanPrefix = (prefix)? prefix : ``
        let result = `${cleanPrefix}${this.name} (parent: ${this.parent?.name})\n`
        cleanPrefix += `\t`
        for (let node of this.children) {
            result += node.toString(cleanPrefix)
        }
        return result
    }
}