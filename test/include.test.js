const cleanup = (fragmentId) => {
    const testingDOMNode = document.getElementById(`TestingDOM`)

    while (testingDOMNode?.firstChild) {
        testingDOMNode.removeChild(testingDOMNode.firstChild)
    }
    if (fragmentId) {
        const componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        const componentTest = document.getElementById(`TestTag${fragmentId}`)
        const componentSyle = document.getElementById(`StyleTag${fragmentId}`)
    
        componentScript?.remove()
        componentTest?.remove()
        componentSyle?.remove()
    }
    window.$components = undefined
    window.initialized = false
    window.beforeMount = false
    window.afterMount = false
}
document.addEventListener('DOMContentLoaded', async () => {
suite(`Test IncludeNode`, `Ensure IncludeNode is working.`, [
    await test (`Create`, `Ensure IncludeNode objects are properly created.`, [async () => {
        let includeNode = new IncludeNode(`TestIncludeNode`)
        let results = []

        assert(includeNode,                                                 `includeNode created.`, results)
        assert(includeNode.name == `TestIncludeNode`,                       `includeNode name set.`, results)
        assert(includeNode.children !== undefined,                          `includeNode children not undefined.`, results)
        assert(includeNode.children !== null,                               `includeNode children not null.`, results)
        assert(includeNode.children.length === 0,                           `includeNode children array is empty.`, results)
        assert(includeNode.parent === null,                                 `includeNode has no parent.`, results)

        return results
    }]),
    await test (`Add child nodes`, `Ensure adding child nodes works correctly.`, [async () => {
        let includeNode = new IncludeNode(`TestIncludeNode`)
        let results = []

        includeNode.addChild(`TestIncludeNodeChild`)
        assert(includeNode.children.length === 1,                           `includeNode children has one child.`, results)
        assert(includeNode.children[0].name === `TestIncludeNodeChild`,     `includeNode child is named TestIncludeNodeChild.`, results)
        assert(includeNode.parent === null,                                 `includeNode has no parent.`, results)
        assert(includeNode.children[0].parent === includeNode,              `Child's parent is includeNode.`, results)
        assert(includeNode.hasChild(`TestIncludeNodeChild`),                `includeNode has child TestIncludeNodeChild.`, results)
        assert(includeNode.getChildByName(`TestIncludeNodeChild`).name === `TestIncludeNodeChild`,
                                                                            `includeNode can return child node object by name.`, results)

        return results                                                                    
    }]),
    await test (`Ancestors`, `Ensure ancestor nodes works correctly.`, [async () => {
        let includeNode = new IncludeNode(`TestIncludeNode`)
        let results = []

        includeNode.addChild(`TestIncludeNodeChild`)
        includeNode.children[0].addChild(`TestIncludeNodeGrandchild`)
        const child = includeNode.children[0]
        const grandchild = child.children[0]

        assert(includeNode.children.length === 1,                           `includeNode children has one child.`, results)
        assert(includeNode.children[0].name ===                             `TestIncludeNodeChild`, `includeNode child is named TestIncludeNodeChild.`, results)
        assert(includeNode.parent === null,                                 `includeNode has no parent.`, results)
        assert(includeNode.children[0].parent === includeNode,              `Child's parent is includeNode.`, results)
        assert(includeNode.hasChild(`TestIncludeNodeChild`),                `includeNode has child TestIncludeNodeChild.`, results)
        assert(includeNode.getChildByName(`TestIncludeNodeChild`).name === `TestIncludeNodeChild`,
                                                                            `includeNode can return child node object by name.`, results)
        assert(child.children.length === 1,                                 `TestIncludeNodeChild children has one child.`, results)
        assert(child.children[0].name ===                                   `TestIncludeNodeGrandchild`, `TestIncludeNodeChild child is named TestIncludeNodeGrandchild.`, results)
        assert(child.parent === includeNode,                                `TestIncludeNodeChild parent is TestIncludeNode.`, results)
        assert(child.children[0].parent === child,                          `Grandchild's parent is TestIncludeNodeChild.`, results)
        assert(child.hasChild(`TestIncludeNodeGrandchild`),                 `TestIncludeNodeChild has child TestIncludeNodeGrandchild.`, results)
        assert(child.getChildByName(`TestIncludeNodeGrandchild`).name === `TestIncludeNodeGrandchild`,
                                                                            `TestIncludeNodeChild can return child node object by name.`, results)
        assert(grandchild.children.length === 0,                            `TestIncludeNodeGrandchild children has no children.`, results)
        assert(grandchild.parent === child,                                 `TestIncludeNodeGrandchild parent is TestIncludeNodeChild.`, results)
        assert(grandchild.hasAncestor(`TestIncludeNodeChild`),              `TestIncludeNodeGrandchild has ancestor TestIncludeNodeChild.`, results)
        assert(grandchild.hasAncestor(`TestIncludeNode`),                   `TestIncludeNodeGrandchild has ancestor TestIncludeNode.`, results)
        assert(grandchild.getAncestor(`TestIncludeNode`) === includeNode,   `TestIncludeNodeGrandchild returns ancestor TestIncludeNodeChild.`, results)
        assert(grandchild.getAncestor(`TestIncludeNodeChild`) === child,    `TestIncludeNodeGrandchild returns ancestor TestIncludeNodeChild.`, results)
        assert(child.hasAncestor(`TestIncludeNode`),                        `TestIncludeNodeChild has ancestor TestIncludeNode.`, results)
        assert(child.getAncestor(`TestIncludeNode`) === includeNode,        `TestIncludeNodeChild returns ancestor TestIncludeNode.`, results)

        return results                                                                    
    }]),
    await test (`Descendants`, `Ensure descendant nodes works correctly.`, [async () => {
        let includeNode = new IncludeNode(`TestIncludeNode`)
        let results = []

        includeNode.addChild(`TestIncludeNodeChild`)
        includeNode.children[0].addChild(`TestIncludeNodeGrandchild`)
        const child = includeNode.children[0]
        const grandchild = child.children[0]

        assert(includeNode.hasDescendant(`TestIncludeNodeChild`),           `TestIncludeNodeGrandchild has ancestor TestIncludeNodeChild.`, results)
        assert(includeNode.hasDescendant(`TestIncludeNodeGrandchild`),      `TestIncludeNodeGrandchild has ancestor TestIncludeNode.`, results)
        assert(includeNode.getDescendant(`TestIncludeNodeChild`) === child, `TestIncludeNodeGrandchild has ancestor TestIncludeNodeChild.`, results)
        assert(includeNode.getDescendant(`TestIncludeNodeGrandchild`) === grandchild,  
                                                                            `TestIncludeNodeGrandchild has ancestor TestIncludeNodeChild.`, results)

        return results                                                                    
    }]),
])

suite(`Test IncludeTree`, `Ensure IncludeTree is working.`, [
    await test (`Create`, `Ensure IncludeTree objects are properly created.`, [async () => {
        const includeTree = new IncludeTree()
        let results = []

        assert(includeTree,                                                 `includeTree created.`, results)
        assert(includeTree.nodes !== undefined,                             `includeTree nodes not undefined.`, results)
        assert(includeTree.nodes !== null,                                  `includeTree nodes not null.`, results)
        assert(includeTree.nodes.length === 0,                              `includeTree nodes array is empty.`, results)

        return results                                                                    
    }]),
    await test (`Add nodes`, `Ensure adding nodes works correctly.`, [async () => {
        const includeTree = new IncludeTree()
        let includeNode = new IncludeNode(`TestIncludeNode`)
        let results = []

        includeTree.addNode(includeNode);
        assert(includeTree.nodes.length === 1,                              `includeTree nodes has one child.`, results)
        assert(includeTree.nodes[0].name === `TestIncludeNode`,             `includeTree node is named TestIncludeNode.`, results)
        assert(includeTree.hasNode(`TestIncludeNode`),                      `includeTree has node TestIncludeNode.`, results)
        assert(includeTree.getNodeByName(`TestIncludeNode`).name === `TestIncludeNode`,
                                                                            `includeTree can return node object by name.`, results)

        return results                                                                    
    }]),
])

suite(`Test ComponentLifecycle`, `Ensure ComponentLifecycle is working.`, [
    await test (`Compile`, `Ensure html is properly compiled.`, [async () => {
        let html = `<div>First Div</div><span>Parent Div<div>Child Div</div></span>`
        let frag = ComponentLifecycle.compile(html)
        let results = []

        assert(frag,                                                        `DOM elements created.`, results)
        assert(frag.children.length === 2,                                  `Correct number of children.`, results)
        assert(frag.children[0].innerText === `First Div`,                  `First div inner text is correct.`, results)
        assert(frag.children[1].innerText === `Parent DivChild Div`,        `Span inner text is correct.`, results)
        assert(frag.children[1].children.length === 1,                      `Span has correct number of children.`, results)
        assert(frag.children[1].children[0].innerText === `Child Div`,      `Child div inner text is correct.`, results)

        return results                                                                    
    }]),
    await test (`Replace Node Values`, `Ensure node values are properly replaced.`, [async () => {
        let text = `Original Text {field1}`
        let textChild = `Original Text {field2}`
        let testDiv = document.createElement('div')
        let testDivText = document.createTextNode(text)
        let testDivChildText = document.createTextNode(textChild)
        let testDivChild = document.createElement('div')
        let testData = { field1: `value 1`, field2: `value 2` }
        let results = []

        testDiv.id = `testDiv`
        testDivChild.id = `testDivChild`
        testDiv.appendChild(testDivText)
        testDiv.appendChild(testDivChild)
        testDivChild.appendChild(testDivChildText)
        document.getElementById(`TestingDOM`).appendChild(testDiv)
        ComponentLifecycle.replaceNodeValue(testDiv, testData, `field1`)
        ComponentLifecycle.replaceNodeValue(testDiv, testData, `field2`)
        
        assert(testDivText.nodeValue === `Original Text value 1`,           `Node value replaced.`, results)
        assert(testDivText.originalNodeValue === text,                      `Original value saved.`, results)
        assert(testDivChildText.nodeValue === `Original Text value 2`,      `Node value replaced in child nodes.`, results)
        assert(testDivChildText.originalNodeValue === textChild,            `Original value saved in child nodes.`, results)

        testDiv.remove()
        testDivChild.remove()

        return results                                                                    
    }]),

    await test (`Replace Attribute Values`, `Ensure attribute values are properly replaced.`, [async () => {
        let text = `Original Text {field1}`
        let textChild = `Original Text {field2}`
        let testDiv = document.createElement('div')
        let testDivChild = document.createElement('div')
        let testDivAttr = document.createAttribute(`test`)
        let testDivChildAttr = document.createAttribute(`test`)
        let testData = { field1: `value 1`, field2: `value 2` }
        let results = []

        testDiv.id = `testDiv`
        testDivChild.id = `testDivChild`
        testDiv.setAttributeNode(testDivAttr)
        testDivChild.setAttributeNode(testDivChildAttr)
        testDivAttr.value = text
        testDivChildAttr.value = textChild
        testDiv.appendChild(testDivChild)
        testDivChild.setAttributeNode(testDivChildAttr)
        document.getElementById(`TestingDOM`).appendChild(testDiv)

        ComponentLifecycle.replaceAttributeValue(testDiv, testData, `field1`)
        ComponentLifecycle.replaceAttributeValue(testDiv, testData, `field2`)
        
        assert(testDivAttr.value === `Original Text value 1`,               `Attribute value replaced.`, results)
        assert(testDivAttr.originalAttributeValue === text,                 `Original value saved.`, results)
        assert(testDivChildAttr.value === `Original Text value 2`,          `Attribute value replaced in child nodes.`, results)
        assert(testDivChildAttr.originalAttributeValue === textChild,       `Original value saved in child nodes.`, results)

        testDiv.remove()
        testDivChild.remove()

        return results                                                                    
    }]),
    await test (`Wrap props`, `Ensure component props are properly wrapped.`, [async () => {
        class TestComponent{
            className(){return this.constructor.name}
            initialize() { if (window.initialized !== undefined) { window.initialized = true }}
            beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true }}
            afterMount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            beforeUnmount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            afterUnmount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            vars = { var1: `TestComponent`, var2: `Y` }
            props = { prop1: `value1`, prop2: `value2` }
        }
        let testComponent = new TestComponent()
        let html = `<footer class="center-text caption-1 red-f p5-tb margin-tb-5 bg-gray-e border-2 border-solid border-black">My Footer</footer>`
        let componentFragment = ComponentLifecycle.compile(html)
        let results = []

        ComponentLifecycle.wrapProps(componentFragment, testComponent)
        testComponent.props.prop1 = `New Value`

        assert(testComponent.props.$propsStore,                             `Data store created.`, results)
        assert(testComponent.props.prop1 === `value1`,                      `Cannot assign value to props.`, results)

        return results                                                                    
    }]),
    await test (`Wrap vars`, `Ensure component vars are properly wrapped.`, [async () => {
        class TestComponent{
            className(){return this.constructor.name}
            initialize() { if (window.initialized !== undefined) { window.initialized = true }}
            beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true }}
            afterMount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            beforeUnmount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            afterUnmount() { if (window.afterMount !== undefined) { window.afterMount = true }}
            vars = { var1: `TestComponent`, var2: `Y` }
            props = { prop1: `value1`, prop2: `value2` }
        }
        let testComponent = new TestComponent()
        let html = `<footer class="center-text caption-1 red-f p5-tb m5-tb bg-gray-e border-2 border-solid border-black">My Footer</footer>`
        let componentFragment = ComponentLifecycle.compile(html)
        let results = []

        ComponentLifecycle.wrapVars(componentFragment, testComponent)
        testComponent.vars.var1 = `New Value`

        assert(testComponent.vars.$varsStore,                               `Data store created.`, results)
        assert(testComponent.vars.var1 === `New Value`,                     `Assigning value to vars works.`, results)

        return results                                                                    
    }]),
    await test (`Register DOM fragment`, `Ensure component DOM fragment is properly registered.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let frag = ComponentLifecycle.compile(html)
        let fragmentId = `TestComponent`
        let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
        let fragmentScripts = frag.querySelectorAll(`script`)
        let fragmentTests = frag.querySelectorAll(`test-script`)
        let fragmentStyles = frag.querySelectorAll(`style`)
        let componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        let componentTest = document.getElementById(`TestTag${fragmentId}`)
        let componentSyle = document.getElementById(`StyleTag${fragmentId}`)
        let componentInRegistry = window.$components?.fragmentRegistry?.has(fragmentId)
        let results = []

        assert(registerResult,                                              `DOM fragment registered.`, results)
        assert(componentInRegistry,                                         `DOM fragment in registry.`, results)
        assert(fragmentScripts.length === 0,                                `Component script moved out of fragment.`, results)
        assert(fragmentTests.length === 0,                                  `Component test moved out of fragment.`, results)
        assert(fragmentStyles.length === 0,                                 `Component style moved out of fragment.`, results)
        assert(componentScript,                                             `Component script still in document.`, results)
        assert(document.head.querySelector(`#ScriptTag${fragmentId}`),      `Component script moved to head.`, results)
        assert(componentTest === null,                                      `Component test not in document.`, results)
        assert(componentSyle,                                               `Component style still in document.`, results)
        assert(document.head.querySelector(`#StyleTag${fragmentId}`),       `Component style moved to head.`, results)

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)
        componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        componentTest = document.getElementById(`TestTag${fragmentId}`)
        componentSyle = document.getElementById(`StyleTag${fragmentId}`)

        assert(registerResult,                                              `DOM fragment registered (include test tag).`, results)
        assert(componentInRegistry,                                         `DOM fragment in registry (include test tag).`, results)
        assert(fragmentScripts.length === 0,                                `Component script moved out of fragment (include test tag).`, results)
        assert(fragmentTests.length === 0,                                  `Component test moved out of fragment (include test tag).`, results)
        assert(fragmentStyles.length === 0,                                 `Component style moved out of fragment (include test tag).`, results)
        assert(componentScript,                                             `Component script still in document (include test tag).`, results)
        assert(document.head.querySelector(`#ScriptTag${fragmentId}`),      `Component script moved to head (include test tag).`, results)
        assert(componentTest,                                               `Component test still in document (include test tag).`, results)
        assert(document.head.querySelector(`#TestTag${fragmentId}`),        `Component test moved to head (include test tag).`, results)
        assert(componentSyle,                                               `Component style still in document (include test tag).`, results)
        assert(document.head.querySelector(`#StyleTag${fragmentId}`),       `Component style moved to head (include test tag).`, results)

        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)

        assert(!registerResult,                                             `Register DOM fragment fails when fragment is already registered.`, results)

        componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        componentTest = document.getElementById(`TestTag${fragmentId}`)
        componentSyle = document.getElementById(`StyleTag${fragmentId}`)
        componentScript?.remove()
        componentTest?.remove()
        componentSyle?.remove()
        window.$components.fragmentRegistry.delete(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(null, frag, false)

        assert(!registerResult,                                             `Register DOM fragment fails when no fragment id is provided.`, results)

        cleanup(fragmentId)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, null, false)

        assert(!registerResult,                                             `Register DOM fragment fails when no fragment is provided.`, results)

        html = `<custom-component><test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's no script tag.`, results)

        html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <script>const f = () => {}</script>
            <test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        
        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's two script tags.`, results)
    
        html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            </custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's no markup tag.`, results)

        html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup>
            <component-markup><div></div></component-markup>
            </custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's two markup tags.`, results)

        html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup>
            </custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)
    
        assert(registerResult,                                              `Register DOM fragment succeeds when including test tag, but there's no test tag.`, results)

        html = `<custom-component><script>
        class TestComponent{
            className() { return this.constructor.name }
            initialize() { if (window.initialized !== undefined) { window.initialized = true } }
            beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
            afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
            beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
            afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
            vars = { var1: 'value1', var2: 'value2' } 
            props = { prop1: 'value3', prop2: 'value4' }
        }</script>
        <test-script>const myTest = function() { return 42; }</test-script>
        <test-script>const myOtherTest = function() { return 42; }</test-script>
        <style>.buttonStyle { color: green; background-color: red; }</style>
        <component-markup>
            <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
            <div id='test-div-1'>{var2}</div>
            <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
        </component-markup></custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's two test tags.`, results)

        html = `<custom-component><script>
        class TestComponent{
            className() { return this.constructor.name }
            initialize() { if (window.initialized !== undefined) { window.initialized = true } }
            beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
            afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
            beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
            afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
            vars = { var1: 'value1', var2: 'value2' } 
            props = { prop1: 'value3', prop2: 'value4' }
        }</script>
        <test-script>const myTest = function() { return 42; }</test-script>
        <style>.buttonStyle { color: green; background-color: red; }</style>
        <style>.checkboxStyle { color: green; background-color: red; }</style>
        <component-markup>
            <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
            <div id='test-div-1'>{var2}</div>
            <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
        </component-markup></custom-component>`

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)
    
        assert(!registerResult,                                             `Register DOM fragment fails when there's two style tags.`, results)

        cleanup(fragmentId)
        return results                                                                    
    }]),
    await test (`Unregister DOM fragment`, `Ensure component DOM fragment is properly unregistered.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <test-script>const myTest = function() { return 42; }</test-script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let frag = ComponentLifecycle.compile(html)
        let fragmentId = `TestComponent`
        let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
        let unregisterResult = ComponentLifecycle.unregisterDOMFragment(fragmentId)
        let componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        let componentTest = document.getElementById(`TestTag${fragmentId}`)
        let componentSyle = document.getElementById(`StyleTag${fragmentId}`)
        let componentInRegistry = window.$components?.fragmentRegistry?.has(fragmentId)
        let results = []

        assert(unregisterResult,                                            `DOM fragment unregistered.`, results)
        assert(!componentInRegistry,                                        `DOM fragment not in registry.`, results)
        assert(componentScript === null,                                    `Component script not in document.`, results)
        assert(componentTest === null,                                      `Component test not in document.`, results)
        assert(componentSyle === null,                                      `Component style still in document.`, results)

        cleanup(fragmentId)
        frag = ComponentLifecycle.compile(html)
        registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, true)
        unregisterResult = ComponentLifecycle.unregisterDOMFragment(fragmentId)
        componentScript = document.getElementById(`ScriptTag${fragmentId}`)
        componentTest = document.getElementById(`TestTag${fragmentId}`)
        componentSyle = document.getElementById(`StyleTag${fragmentId}`)
        componentInRegistry = window.$components?.fragmentRegistry?.has(fragmentId)

        assert(registerResult,                                              `DOM fragment registers successfully after an unregistered.`, results)
        assert(unregisterResult,                                            `DOM fragment unregistered (include test tag).`, results)
        assert(!componentInRegistry,                                        `DOM fragment not in registry (include test tag).`, results)
        assert(componentScript === null,                                    `Component script not in document (include test tag).`, results)
        assert(componentTest === null,                                      `Component test not in document (include test tag).`, results)
        assert(componentSyle === null,                                      `Component style still in document (include test tag).`, results)

        cleanup(fragmentId)
        return results                                                                    
    }]),
    await test (`Create component object`, `Ensure a component's object can be successfully created.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent {
                className() { return this.constructor.name }
                initialize() { window.initialized = true }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        const setup = (fragmentId, html, includeTagHTML) => {
            let frag = ComponentLifecycle.compile(html)
            let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
            let testingDOMNode = document.getElementById(`TestingDOM`)
    
            assert(registerResult,                                          `DOM fragment registered successfully.`, results)
            window.initialized = false
            testingDOMNode.append(...new DOMParser().parseFromString(includeTagHTML, `text/html`).body.childNodes)
        }
        let results = []
        let fragmentId = `TestComponent`
        let testIncludeTagId = `TestIncludeTag`
        let includeTagHTML = `<include-html id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`
        
        setup(fragmentId, html, includeTagHTML)

        let componentObject = ComponentLifecycle.createComponentObject(fragmentId, `TestComponent1`, document.getElementById(testIncludeTagId))
        let objectInRegistry = window.$components?.objectRegistry?.has(fragmentId)
        let hasIncludeTag = document.getElementById(testIncludeTagId)
        let hasMarkerTag = document.getElementById(`-ComponentBeginMarkerTestComponent1`)

        componentObject.props.prop1 = "Some value"

        assert(componentObject,                                             `Component object successfully created.`, results)
        assert(!objectInRegistry,                                           `Creating a component does not register it.`, results)
        assert(!hasIncludeTag,                                              `Include tag removed from document after object creation.`, results)
        assert(hasMarkerTag,                                                `Marker tag added to document after object creation.`, results)
        assert(window.initialized,                                          `Component has been initialized.`, results)
        assert(componentObject.vars.$varsStore,                             `Vars are wrapped.`, results)
        assert(componentObject.props.$propsStore,                           `Props are wrapped.`, results)
        assert(componentObject.props.prop1 === `value3`,                    `Props cannot be changed.`, results)
        
        cleanup(fragmentId)
        includeTagHTML = `<include-html props='{"q":"Q"}' id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`
        setup(fragmentId, html, includeTagHTML)
        componentObject = ComponentLifecycle.createComponentObject(fragmentId, `TestComponent1`, document.getElementById(testIncludeTagId))
        objectInRegistry = window.$components?.objectRegistry?.has(fragmentId)
        hasIncludeTag = document.getElementById(`TestIncludeTag`)

        assert(componentObject,                                             `Component object successfully created.`, results)
        assert(componentObject.props.$propsStore,                           `Props are wrapped.`, results)
        assert(componentObject.props.prop1 === `value3`,                    `Props cannot be changed.`, results)
        assert(componentObject.props.q === `Q`,                             `Props can be set via the include tag.`, results)

        cleanup(fragmentId)
        includeTagHTML = `<include-html vars='{"q":"Q"}' id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`
        setup(fragmentId, html, includeTagHTML)
        componentObject = ComponentLifecycle.createComponentObject(fragmentId, `TestComponent1`, document.getElementById(testIncludeTagId))
        objectInRegistry = window.$components?.objectRegistry?.has(fragmentId)

        assert(componentObject,                                             `Component object successfully created.`, results)
        assert(!objectInRegistry,                                           `Creating a component does not register it.`, results)
        assert(componentObject.vars.$varsStore,                             `Vars are wrapped.`, results)
        assert(componentObject.vars.q === `Q`,                              `Vars can be set via the include tag.`, results)

        cleanup(fragmentId)
        return results                                                                    
    }]),
    await test (`Register component object`, `Ensure a component's object can be successfully registered.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent {
                className() { return this.constructor.name }
                initialize() { window.initialized = true }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let results = []
        const setup = (fragmentId, html, includeTagHTML) => {
            let frag = ComponentLifecycle.compile(html)
            let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
            let testingDOMNode = document.getElementById(`TestingDOM`)
    
            assert(registerResult,                                          `DOM fragment registered successfully.`, results)
            window.initialized = false
            testingDOMNode.append(...new DOMParser().parseFromString(includeTagHTML, `text/html`).body.childNodes)
        }
        let fragmentId = `TestComponent`
        let componentObjectID = `TestComponent1`
        let includeTagHTML = `<include-html id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`

        setup(fragmentId, html, includeTagHTML)

        let registerComponentObjectResult = ComponentLifecycle.registerComponentObject(fragmentId, componentObjectID, { test: `test` })
        let objectInRegistry = window.$components?.objectRegistry?.has(componentObjectID)

        assert(registerComponentObjectResult,                               `Component object registered successfully.`, results)
        assert(objectInRegistry,                                            `Component object in registry.`, results)

        registerComponentObjectResult = ComponentLifecycle.registerComponentObject(fragmentId, componentObjectID, { test: `test` })

        assert(!registerComponentObjectResult,                              `Registering an object fails when it is already registered.`, results)

        cleanup(fragmentId)
        setup(fragmentId, html, includeTagHTML)
        registerComponentObjectResult = ComponentLifecycle.registerComponentObject(fragmentId, null, { test: `test` })

        assert(!registerComponentObjectResult,                              `Registering an object fails when no component object id is provided.`, results)

        cleanup(fragmentId)
        setup(fragmentId, html, includeTagHTML)
        registerComponentObjectResult = ComponentLifecycle.registerComponentObject(fragmentId, componentObjectID, null)

        assert(!registerComponentObjectResult,                              `Registering an object fails when no component object is provided.`, results)

        cleanup(fragmentId)
        setup(fragmentId, html, includeTagHTML)
        registerComponentObjectResult = ComponentLifecycle.registerComponentObject(null, componentObjectID, { test: `test` })

        assert(!registerComponentObjectResult,                              `Registering an object fails when no component class is provided.`, results)

        cleanup(fragmentId)

        return results                                                                    
    }]),
    await test (`Unregister component object`, `Ensure a component's object can be successfully unregistered.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent {
                className() { return this.constructor.name }
                initialize() { window.initialized = true }
                beforeMount() { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let results = []
        const setup = (fragmentId, html, includeTagHTML) => {
            let frag = ComponentLifecycle.compile(html)
            let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
            let testingDOMNode = document.getElementById(`TestingDOM`)
    
            assert(registerResult,                                          `DOM fragment registered successfully.`, results)
            window.initialized = false
            testingDOMNode.append(...new DOMParser().parseFromString(includeTagHTML, `text/html`).body.childNodes)
        }
        let fragmentId = `TestComponent`
        let componentObjectID = `TestComponent1`
        let includeTagHTML = `<include-html id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`

        setup(fragmentId, html, includeTagHTML)

        let registerComponentObjectResult = ComponentLifecycle.registerComponentObject(`fragment`, `TestObject`, { test: `test` })
        let unregisterComponentObjectResult = ComponentLifecycle.unregisterComponentObject(`TestObject`)
        let objectInRegistry = window.$components?.objectRegistry?.has(`TestObject`)

        assert(registerComponentObjectResult,                               `Component object registered successfully.`, results)
        assert(unregisterComponentObjectResult,                             `Component object unregistered successfully.`, results)
        assert(!objectInRegistry,                                           `Component object in registry.`, results)

        unregisterComponentObjectResult = ComponentLifecycle.unregisterComponentObject(`fragment`, `TestObject`, { test: `test` })

        assert(!unregisterComponentObjectResult,                            `Unregistering an object fails when it is already unregistered.`, results)

        cleanup(fragmentId)
        setup(fragmentId, html, includeTagHTML)
        unregisterComponentObjectResult = ComponentLifecycle.registerComponentObject()

        assert(!unregisterComponentObjectResult,                            `Unegistering an object fails when no component object id is provided.`, results)

        cleanup(fragmentId)

        return results                                                                    
    }]),
    await test (`Mount component`, `Ensure a component can be successfully mounted.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let results = []
        const setupComponent = (testIncludeTagId, componentClass, componentObjectId) => {
            let componentObject = ComponentLifecycle.createComponentObject(componentClass, componentObjectId, document.getElementById(testIncludeTagId))
            let registerComponentObjectResult = ComponentLifecycle.registerComponentObject(componentClass, componentObjectId, componentObject)
            assert(registerComponentObjectResult,                           `Component object registered successfully.`, results)
        }
        const setup = (fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, componentObjectId) => {
            let frag = ComponentLifecycle.compile(html)
            let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
            let testingDOMNode = document.getElementById(`TestingDOM`)
    
            assert(registerResult,                                          `DOM fragment registered successfully.`, results)
            window.beforeMount = false
            window.afterMount = false
            testingDOMNode.append(...new DOMParser().parseFromString(includeTagHTML, `text/html`).body.childNodes)
            setupComponent(testIncludeTagId, componentClass, componentObjectId)
         }
        let fragmentId = `TestComponent`
        let testIncludeTagId = `TestIncludeTag`
        let componentClass = `TestComponent`
        let componentObjectId = `TestComponent1`
        let includeTagHTML = `<include-html id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`
        setup(fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, componentObjectId)
        let mountResult = ComponentLifecycle.mount(componentObjectId)
        let componentObjectInfo = window.$components.objectRegistry.get(componentObjectId)

        assert(mountResult,                                                 `Component was mounted.`, results)
        assert(window.beforeMount,                                          `Component's beforeMount() method was called.`, results)
        assert(window.afterMount,                                           `Component's afterMount() method was called.`, results)
        assert(componentObjectInfo?.mounted,                                `Component's marked as mounted.`, results)

        cleanup(fragmentId)
        setup(fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, componentObjectId)
        mountResult = ComponentLifecycle.mount()

        assert(!mountResult,                                                `Mount fails when no id is provided.`, results)

        cleanup(fragmentId)
        setup(fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, `WrongComponentObjectId`)
        mountResult = ComponentLifecycle.mount(componentObjectId)

        assert(!mountResult,                                                `Mount fails when component is not registered.`, results)

        cleanup(fragmentId)
        setup(fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, componentObjectId)
        window.$components.objectRegistry.set(componentObjectId, componentObjectInfo)

        let framentRegisryObject = window.$components.fragmentRegistry.get(componentClass)

        window.$components.fragmentRegistry.delete(componentClass)
        mountResult = ComponentLifecycle.mount(componentObjectId)

        assert(!mountResult,                                                `Mount fails when fragment is not registered.`, results)

        cleanup(fragmentId)
        setup(fragmentId, testIncludeTagId, html, includeTagHTML, componentClass, componentObjectId)
        window.$components.fragmentRegistry.set(componentClass, framentRegisryObject)

        let markerElement = document.getElementById(`-ComponentBeginMarkerTestComponent1`)

        markerElement.remove()
        mountResult = ComponentLifecycle.mount(`TestComponent1`)

        assert(!mountResult,                                                `Mount fails when marker tag is not in DOM.`, results)

        let testingDOMNode = document.getElementById(`TestingDOM`)

        cleanup(fragmentId)
        return results                                                                    
    }]),
    await test (`Unmount component`, `Ensure a component can be successfully unmounted.`, [async () => {
        let html = `<custom-component><script>
            class TestComponent{
                className() { return this.constructor.name }
                initialize() { if (window.initialized !== undefined) { window.initialized = true } }
                beforeMount() { { if (window.beforeMount !== undefined) { window.beforeMount = true } }
                afterMount() { { if (window.afterMount !== undefined) { window.afterMount = true } }
                beforeUnmount() { if (window.beforeUnmount !== undefined) { window.beforeUnmount = true } }
                afterUnmount() { if (window.afterUnmount !== undefined) { window.afterUnmount = true } }
                vars = { var1: 'value1', var2: 'value2' } 
                props = { prop1: 'value3', prop2: 'value4' }
            }</script>
            <style>.buttonStyle { color: green; background-color: red; }</style>
            <component-markup>
                <button id='test-button' name="{var2}" class="buttonStyle" onclick="console.log('clicked')">{var1}</button>
                <div id='test-div-1'>{var2}</div>
                <div id='test-div-2'>{prop1}<div id='test-div-3'>{prop2}</div></div>
            </component-markup></custom-component>`
        let includeTagHTML = `<include-html id="TestIncludeTag" component="TestComponent" component-id="TestComponent1" src="./components/not-used-for-this-test.html"></include-html>`
        let frag = ComponentLifecycle.compile(html)
        let fragmentId = `TestComponent`
        let registerResult = ComponentLifecycle.registerDOMFragment(fragmentId, frag, false)
        let testingDOMElement = document.getElementById(`TestingDOM`)
        let results = []

        window.beforeUnmount = false
        window.afterUnmount = false
        testingDOMElement.append(...new DOMParser().parseFromString(includeTagHTML, `text/html`).body.childNodes)

        let componentObject = ComponentLifecycle.createComponentObject(fragmentId, `TestComponent1`, document.getElementById(`TestIncludeTag`))
        let registerComponentObjectResult = ComponentLifecycle.registerComponentObject(fragmentId, `TestComponent1`, componentObject)
        let mountResult = ComponentLifecycle.mount(`TestComponent1`)
        let unmountResult = ComponentLifecycle.unmount(`TestComponent1`)
        let componentObjectInfo = window.$components.objectRegistry.get(`TestComponent1`)

        assert(unmountResult,                                               `Component was unmounted.`, results)
        assert(window.beforeUnmount,                                        `Component's beforeUnmount() method was called.`, results)
        assert(window.afterUnmount,                                         `Component's afterUnmount() method was called.`, results)
        assert(!componentObjectInfo?.mounted,                               `Component marked as unmounted.`, results)

        unmountResult = ComponentLifecycle.unmount()

        assert(!unmountResult,                                              `Unmount fails when no id is provided.`, results)

        window.$components.objectRegistry.delete(`TestComponent1`)
        unmountResult = ComponentLifecycle.unmount()

        assert(!unmountResult,                                              `Unmount fails when component is not registered.`, results)

        window.$components.objectRegistry.set(`TestComponent1`, componentObjectInfo)

        let framentRegisryObject = window.$components.fragmentRegistry.get(fragmentId)

        window.$components.fragmentRegistry.delete(fragmentId)
        unmountResult = ComponentLifecycle.unmount(`TestComponent1`)

        assert(!unmountResult,                                              `Unmount fails when fragment is not registered.`, results)

        window.$components.fragmentRegistry.set(fragmentId, framentRegisryObject)

        let markerElement = document.getElementById(`-ComponentBeginMarkerTestComponent1`)

        markerElement.remove()
        unmountResult = ComponentLifecycle.unmount(`TestComponent1`)

        assert(!unmountResult,                                              `Unmount fails when marker tag is not in DOM.`, results)

        let testingDOMNode = document.getElementById(`TestingDOM`)

        cleanup(fragmentId)
        return results                                                                    
    }]),
])
suite(`Test Vanilla`, `Ensure Vanilla utility class is working.`, [
    await test (`Get component fragment`, `Ensure component fragments can be retrieved from the component fragment registry.`, [async () => {
        window.$components = {}
        window.$components.fragmentRegistry = new Map()
        window.$components.fragmentRegistry.set(`TestFragment`, { data: 'data'})

        let fragment =  Component.getComponentFragment(`TestFragment`)
        let results = []

        assert(fragment,                                                    `An object was retrieved from the component fragment registry.`, results)
        assert(fragment.data === 'data',                                    `Correct object retrieved from the component fragment registry.`, results)

        fragment =  Component.getComponentFragment()

        assert(!fragment,                                                   `Get component fragment fails when no id is provided.`, results)

        window.$components.fragmentRegistry.delete(`TestFragment`)
        fragment =  Component.getComponentFragment(`TestFragment`)

        assert(!fragment,                                                   `Get component fragment fails when object not in the component fragment registry.`, results)

        return results                                                                    
    }]),
    await test (`Get component object`, `Ensure component objects can be retrieved from the component object registry.`, [async () => {
        window.$components = {}
        window.$components.objectRegistry = new Map()
        window.$components.objectRegistry.set(`TestObject`, { componentObject: { data: 'data'}})

        let object =  Component.getObject(`TestObject`)
        let results = []

        assert(object,                                                      `An object was retrieved from the component object registry.`, results)
        assert(object.data === 'data',                                      `Correct object retrieved from the component object registry.`, results)

        object =  Component.getObject()

        assert(!object,                                                     `Get component object fails when no id is provided.`, results)

        window.$components.objectRegistry.delete(`TestObject`)
        object =  Component.getComponentFragment(`TestObject`)

        assert(!object,                                                     `Get component object fails when object not in the component object registry.`, results)

        return results                                                                    
    }]),
])
suite(`Test Loader`, `Ensure Loader correctly processes include files.`, [
    await test (`Load file`, `Ensure files can be loaded.`, [async () => {
        let file = `./support-files/text.txt`
        let text = await Loader.loadFile(file)
        let results = []

        assert(text === `Text`,                                             `Text read from file.`, results)

        try {
            text = await Loader.loadFile(`no-such.file`)
        } catch (e) {
            assert(true,                                                    `Loading non-existant file throws an error.`, results)
        }

        return results                                                                    
    }]),    
    await test (`Update include tree`, `Ensure include tree works correctly.`, [async () => {
        Loader.includeTree = new IncludeTree()

        let results = []
        let newChildNode = Loader.updateIncludeTree(`parent`, `child`)

        assert(newChildNode,                                                `Include tree was successfully updated.`, results)
        assert(Loader.tree.nodes.length === 1,                              `Node added to include tree.`, results)
        assert(Loader.tree.nodes[0].name === `parent`,                      `Parent added to include tree.`, results)

        let parent = Loader.includeTree.getNodeByName(`parent`)
        let child = Loader.includeTree.getNodeByName(`child`)

        assert(parent,                                                      `Parent is in the tree.`, results)
        assert(child,                                                       `Child is in the tree.`, results)
        assert(!parent.parent,                                              `Parent does not have a parent.`, results)
        assert(parent.children.length === 1,                                `Parent has one child.`, results)
        assert(parent.children[0] === child,                                `Parent's child is the child node.`, results)
        assert(child.parent,                                                `Child has a parent.`, results)
        assert(child.parent === parent,                                     `Child's parent is the parent node.`, results)
        assert(child.children.length === 0,                                 `Child has no children.`, results)

        newChildNode = Loader.updateIncludeTree(`parent`, `child2`)

        let child2 = Loader.includeTree.getNodeByName(`child2`)

        assert(newChildNode,                                                `Node added to the tree.`, results)
        assert(child2,                                                      `Child2 is in the tree.`, results)
        assert(parent.children.length === 2,                                `Parent has two children.`, results)
        assert(parent.children[1] === child2,                               `Parent's second child is the child2 node.`, results)
        assert(child2.parent,                                               `Child2 has a parent.`, results)
        assert(child2.parent === parent,                                    `Child2's parent is the parent node.`, results)

        newChildNode = Loader.updateIncludeTree(`parent2`, `child3`)

        let parent2 = Loader.includeTree.getNodeByName(`parent2`)
        let child3 = Loader.includeTree.getNodeByName(`child3`)

        assert(Loader.tree.nodes.length === 2,                              `Node added to tree.`, results)
        assert(newChildNode,                                                `Node added to the tree.`, results)
        assert(parent2,                                                     `Parent2 is in the tree.`, results)
        assert(child3,                                                      `Child3 is in the tree.`, results)
        assert(parent2.children.length === 1,                               `Parent2 has one child.`, results)
        assert(parent2.children[0] === child3,                              `Parent2's child is the child3 node.`, results)
        assert(child3.parent,                                               `Child3 has a parent.`, results)
        assert(child3.parent === parent2,                                   `Child3's parent is the parent2 node.`, results)

        newChildNode = Loader.updateIncludeTree(`child`, `grandchild`)

        let grandchild = Loader.includeTree.getNodeByName(`grandchild`)

        assert(newChildNode,                                                `Node added to the tree.`, results)
        assert(grandchild,                                                  `Grandchild added to the tree.`, results)
        assert(child.children.length === 1,                                 `Child has one child.`, results)
        assert(child.children[0] === grandchild,                            `Child's child is the grandchild node.`, results)
        assert(grandchild.parent,                                           `Grandchild has a parent.`, results)
        assert(grandchild.parent === child,                                 `Grandchild's parent is the child node.`, results)

        newChildNode = Loader.updateIncludeTree(`child`, `parent`)

        assert(!newChildNode,                                               `Node not added to the tree when it causes recursion.`, results)

        newChildNode = Loader.updateIncludeTree(`child2`, `parent`)

        assert(!newChildNode,                                               `Node not added to the tree when it causes recursion.`, results)

        newChildNode = Loader.updateIncludeTree(`grandchild`, `parent`)

        assert(!newChildNode,                                               `Node not added to the tree when it causes recursion.`, results)

        return results                                                                    
    }]),
    await test (`Validate include attributes`, `Ensure include tag attributes are correctly validated.`, [async () => {
        let results = []
        let div = document.createElement('div')

        div.setAttribute(`src`, `src value`)
        div.setAttribute(`include-in`, `include-in value`)

        let [src, includeIn, componentClass, componentObjectId, repeat] = Loader.validateIncludeAttributes(div.attributes)

        assert(src == `src value`,                                          `Src value read correctly.`, results)
        assert(includeIn == `include-in value`,                             `Include-in value read correctly.`, results)
        assert(componentClass === undefined,                                `No component-class attribute handled correctly.`, results)
        assert(componentObjectId === undefined,                             `No component-id attribute handled correctly.`, results)
        assert(repeat === 1,                                                `Repeat defaults to 1.`, results)

        div.removeAttribute(`src`)

        let [src1, includeIn1, componentClass1, componentObjectId1, repeat1] = Loader.validateIncludeAttributes(div.attributes)

        assert(src1 == null,                                                `Src is null when src is missing.`, results)
        assert(includeIn1 == null,                                          `Include-in is null when src is missing.`, results)
        assert(componentClass1 === null,                                    `component-class is null when src is missing.`, results)
        assert(componentObjectId1 === null,                                 `component-id is null when src is missing.`, results)
        assert(repeat1 === null,                                            `Repeat is null when src is missing.`, results)

        div.setAttribute(`src`, `src value`)
        div.removeAttribute(`include-in`)

        let [src2, includeIn2, componentClass2, componentObjectId2, repeat2] = Loader.validateIncludeAttributes(div.attributes)

        assert(src2 == null,                                                `Src is null when include-in is missing.`, results)
        assert(includeIn2 == null,                                          `Include-in is null when include-in is missing.`, results)
        assert(componentClass2 === null,                                    `Component-class is null when include-in is missing.`, results)
        assert(componentObjectId2 === null,                                 `Component-id is null when include-in is missing.`, results)
        assert(repeat2 === null,                                            `Repeat is null when include-in is missing.`, results)

        div.setAttribute(`include-in`, `include-in value`)
        div.setAttribute(`component-class`, `component-class value`)
        div.setAttribute(`component-id`, `component-id value`)

        let [src3, includeIn3, componentClass3, componentObjectId3, repeat3] = Loader.validateIncludeAttributes(div.attributes)

        assert(src3 == `src value`,                                         `Src value read correctly.`, results)
        assert(includeIn3 == `include-in value`,                            `Include-in value read correctly.`, results)
        assert(componentClass3 === `component-class value`,                 `Component-class value read correctly.`, results)
        assert(componentObjectId3 === `component-id value`,                 `Component-id value read correctly.`, results)
        assert(repeat3 === 1,                                               `Repeat defaults to 1.`, results)

        div.removeAttribute(`component-class`)

        let [src4, includeIn4, componentClass4, componentObjectId4, repeat4] = Loader.validateIncludeAttributes(div.attributes)

        assert(src4 == null,                                                `Src is null when component-class is missing and component-id exists.`, results)
        assert(includeIn4 == null,                                          `Include-in is null when component-class is missing and component-id exists.`, results)
        assert(componentClass4 === null,                                    `component-class is null when component-class is missing and component-id exists.`, results)
        assert(componentObjectId4 === null,                                 `component-id is null when component-class is missing and component-id exists.`, results)
        assert(repeat4 === null,                                            `Repeat is null when component-class is missing and component-id exists.`, results)

        div.setAttribute(`component-class`, `component-class value`)
        div.removeAttribute(`component-id`)

        let [src5, includeIn5, componentClass5, componentObjectId5, repeat5] = Loader.validateIncludeAttributes(div.attributes)

        assert(src5 == null,                                                `Src is null when component-class is exists and component-id is missing.`, results)
        assert(includeIn5 == null,                                          `Include-in is null when component-class exists and component-id is missing.`, results)
        assert(componentClass5 === null,                                    `component-class is null when component-class exists and component-id is missing.`, results)
        assert(componentObjectId5 === null,                                 `component-id is null when component-class exists and component-id is missing.`, results)
        assert(repeat5 === null,                                            `Repeat is null when component-class exists and component-id is missing.`, results)

        div.removeAttribute(`component-class`)
        div.removeAttribute(`component-id`)
        div.setAttribute(`repeat`, `5`)

        let [src6, includeIn6, componentClass6, componentObjectId6, repeat6] = Loader.validateIncludeAttributes(div.attributes)

        assert(src6 == `src value`,                                         `Src value read correctly.`, results)
        assert(includeIn6 == `include-in value`,                            `Include-in value read correctly.`, results)
        assert(componentClass6 === undefined,                               `No component-class attribute handled correctly.`, results)
        assert(componentObjectId6 === undefined,                            `No component-id attribute handled correctly.`, results)
        assert(repeat6 === 5,                                               `Repeat attribute read correctly.`, results)

        div.setAttribute(`repeat`, `JUNK`)

        let [src7, includeIn7, componentClass7, componentObjectId7, repeat7] = Loader.validateIncludeAttributes(div.attributes)

        assert(src7 == null,                                                `Src is null when repeat is NaN.`, results)
        assert(includeIn7 == null,                                          `Include-in is null when repeat is NaN.`, results)
        assert(componentClass7 === null,                                    `component-class is null when repeat is NaN.`, results)
        assert(componentObjectId7 === null,                                 `component-id is null when repeat is NaN.`, results)
        assert(repeat7 === null,                                            `Repeat is null when repeat is NaN.`, results)

        div.setAttribute(`repeat`, `0`)

        let [src8, includeIn8, componentClass8, componentObjectId8, repeat8] = Loader.validateIncludeAttributes(div.attributes)

        assert(src8 == null,                                                `Src is null when repeat is less than 1.`, results)
        assert(includeIn8 == null,                                          `Include-in is null when repeat is less than 1.`, results)
        assert(componentClass8 === null,                                    `component-class is null when repeat is less than 1.`, results)
        assert(componentObjectId8 === null,                                 `component-id is null when repeat is less than 1.`, results)
        assert(repeat8 === null,                                            `Repeat is null when repeat is less than 1.`, results)

        return results                                                                    
    }]),    
    await test (`Load include`, `Ensure basic include tags are loaded correctly.`, [async () => {
        let results = []
        let include = document.createElement('div')
        let testingDOMNode = document.getElementById(`TestingDOM`)

        include.id = `include-here`
        include.setAttribute(`src`, `./support-files/footer.html`)
        include.setAttribute(`include-in`, `include.test.js`)

        testingDOMNode.appendChild(include)
        await Loader.loadInclude(include)

        let footer = testingDOMNode.querySelector(`footer`)
        let includeCheck = testingDOMNode.querySelector(`div`)

        assert(footer,                                                      `Footer added to document.`, results)
        assert(!includeCheck,                                               `Include tag removed from document.`, results)

        footer.remove()
        include.setAttribute(`src`, `./support-files/footer.html`)
        include.setAttribute(`include-in`, `looping-footer.js`)
        include.id = `include-here`
        testingDOMNode.appendChild(include)
        await Loader.loadInclude(include)
        footer = testingDOMNode.querySelector(`footer`)

        assert(footer,                                                      `Loader does not load nested includes.`, results)

        footer.remove()
        include.removeAttribute(`src`)
        include.id = `include-here`
        testingDOMNode.appendChild(include)
        await Loader.loadInclude(include)
        footer = testingDOMNode.querySelector(`footer`)

        assert(!footer,                                                     `Loader does not include when src attribute is missing.`, results)

        include.setAttribute(`src`, `./support-files/footer.html`)
        include.removeAttribute(`include-in`)
        include.id = `include-here`
        testingDOMNode.appendChild(include)
        await Loader.loadInclude(include)
        footer = testingDOMNode.querySelector(`footer`)

        assert(!footer,                                                     `Loader does not include when include-in attribute is missing.`, results)

        window.$components = undefined
        while (testingDOMNode.firstChild) {
            testingDOMNode.removeChild(testingDOMNode.firstChild)
        }

        return results                                                                    
    }]),    
    await test (`Load include component`, `Ensure component include tags are loaded correctly.`, [async () => {
        let results = []
        let include = document.createElement('div')
        let testingDOM = document.getElementById(`TestingDOM`)

        include.id = `include-here`
        include.setAttribute(`src`, `./support-files/test-button-component.html`)
        include.setAttribute(`include-in`, `include.test.js`)
        include.setAttribute(`component-class`, `Button`)
        include.setAttribute(`component-id`, `Button1`)
        testingDOM.appendChild(include)
        await Loader.loadInclude(include)

        assert(testingDOM.children[0],                                      `An element was inserted.`, results)
        assert(testingDOM.children[0].tagName == 'SCRIPT',                  `The marker tag was inserted`, results)
        assert(testingDOM.children[1],                                      `An second element was inserted.`, results)
        assert(testingDOM.children[1].tagName == 'BUTTON',                  `The component's button tag was inserted.`, results)
        assert(testingDOM.children[1].innerText == 'Button',                `Button's inner text replaced with the value of var x.`, results)
        assert(testingDOM.children[2],                                      `An third element was inserted.`, results)
        assert(testingDOM.children[2].tagName == 'DIV',                     `The component's first div tag was inserted.`, results)
        assert(testingDOM.children[2].innerText == 'Y',                     `First div's inner text replaced with the value of var y.`, results)
        assert(testingDOM.children[3],                                      `An fourth element was inserted.`, results)
        assert(testingDOM.children[3].tagName == 'DIV',                     `The component's second div tag was inserted.`, results)
        assert(testingDOM.children[3].innerText.indexOf(`value1`) === 0,    `First div's inner text replaced with the value of prop prop1.`, results)
        assert(testingDOM.children.length == 4,                             `There are 4 'top level' elements for the component.`, results)
        assert(testingDOM.children[3].children[0].tagName == `DIV`,         `The second div tag has a child.`, results)
        assert(testingDOM.children[3].children[0].innerText == `value2`,    `The second div's inner text replaced with the value of prop prop2.`, results)
        assert(testingDOM.children[3].children.length == 1,                 `The second div tag has only 1 child.`, results)

        window.$components = undefined
        while (testingDOM.firstChild) {
            testingDOM.removeChild(testingDOM.firstChild)
        }

        return results                                                                    
    }]),    
])
})
