class Loader {
    static msgs = {
        INCLUDES_LOADED: `INCLUDES_LOADED`
    }
    static includeTree = new Tree()
    static includeCache = new Map()
    static get tree() { return Loader.includeTree }
    static get cache() { return Loader.includeCache }
    
    static loadFile = async (filename) => {
        if (Loader.cache.has(filename)) { return Loader.cache.get(filename) }
        let response = await fetch(filename)
    
        if (!response.ok) {
            let error = `loadFile: Network response was not OK while loading ${filename}.`
    
            console.error(error)
            throw new Error(error);
        }
    
        let text = await response.text()

        Loader.cache.set(filename, text)
        return text
    }
    static updateIncludeTree = (parentName, childName) => {
        let node = null
        
        if (Loader.tree.hasNode(parentName)) {
            node = Loader.tree.getNodeByName(parentName)
        } else {
            node = new TreeNode(parentName)
            Loader.tree.addNode(node)
        }
        if (node.hasAncestor(childName)) {
            console.error(`updateIncludeTree: Include-html tag causes infinite recursion. Include processing halted. Parent name is ${parentName}. Child name is ${childName}`)
            return null
        }
        let childNode = node.addChild(childName)
        return childNode
    }
    static validateIncludeAttributes = (attributes) => {
        const badReturn = [null, null, null, null, null]

        if (!attributes) {
            console.error(`validateIncludeAttributes: Include missing required attributes 'include-in' and 'src'. Include processing halted.`)
            return badReturn
        }

        let src = attributes.src?.value
        let includeIn = attributes[`include-in`]?.value
        let componentClass = attributes[`component-class`]?.value
        let componentObjectId = attributes[`component-id`]?.value  
        let repeatAttributValue = attributes[`repeat`]?.value
        let repeat = (repeatAttributValue)? parseInt(repeatAttributValue) : 1

        if (!src) {
            console.error(`validateIncludeAttributes: Include-html tag missing required attribute 'src'. Include processing halted. File containing bad Include-html tag is ${includeIn}.`)
            return badReturn
        }
        if (!includeIn) {
            console.error(`validateIncludeAttributes: Include-html tag missing required attribute 'include-in'. Include processing halted. Included file is ${src}.`)
            return badReturn
        }
        if (componentClass && !componentObjectId) {
            console.error(`validateIncludeAttributes: Include-html tag missing required attribute 'component-id'. Include processing halted. File containing bad Include-html tag is ${includeIn}. Included file is ${src}.`)
            return badReturn
        }
        if (!componentClass && componentObjectId) {
            console.error(`validateIncludeAttributes: Include-html tag missing required attribute 'component-class'. Include processing halted. File containing bad Include-html tag is ${includeIn}. Included file is ${src}.`)
            return badReturn
        }
        if (0 !== repeat && !repeat) {
            console.error(`validateIncludeAttributes: Include-html tag 'repeat' attribute is not a number. Include processing halted. File containing bad Include-html tag is ${includeIn}. Included file is ${src}.`)
            return badReturn
        }
        if (1 > repeat) {
            console.error(`validateIncludeAttributes: Include-html tag 'repeat' attribute must be greater than zero. Include processing halted. File containing bad Include-html tag is ${includeIn}. Included file is ${src}.`)
            return badReturn
        }
        return [src, includeIn, componentClass, componentObjectId, repeat]
    }
    static loadInclude = async function (include, keepTestingScript = false) {
        let [src, includeIn, componentClass, componentObjectId, repeat] = Loader.validateIncludeAttributes(include.attributes)

        if (!src || !includeIn) { return false }

        let nodeAddedToIncludeTree = Loader.updateIncludeTree(includeIn, src)

        if (!nodeAddedToIncludeTree) { return false }

        let text = await Loader.loadFile(include.attributes.src.value)

        for (let loop = 0; loop < repeat; loop++) {
            if (!componentClass) {
                include.insertAdjacentHTML('afterend', text)
            } else {
                let loadIncludeComponentResult = await Loader.loadIncludeComponent(text, src, includeIn, componentClass, componentObjectId, include, keepTestingScript)
                if (!loadIncludeComponentResult) { return false }
            }
        }

        return true
    }
    static registerChildComponents = (fragment, componentClass) => {
        const componentMarkupTag = fragment.querySelector("component-markup")
        const includeHTMLTags = componentMarkupTag?.querySelectorAll("include-html")

        if (!window.$components.childComponentRegistry) { window.$components.childComponentRegistry = new Map() }

        let data = { component: componentClass, childComponents: []}

        for (let loop = 0; loop < includeHTMLTags.length; loop++) {
            let includeHTMLTag = includeHTMLTags[loop]

            if (includeHTMLTag.hasAttribute(`component-id`)) { data.childComponents.push(includeHTMLTag.getAttribute(`component-id`).replace(`{id}`, ``)) }
        }
        window.$components.childComponentRegistry.set(data.component, data.childComponents)
    }
    static addChildComponentGettersToComponentObject(componentClass, componentObjectId) {
        const childComponents = window.$components.childComponentRegistry.get(componentClass)
        const componentObject = Component.getObject(componentObjectId)

        if (!childComponents || !componentObject) { return }
        for (let childComponentId of childComponents) {
            let getterName = childComponentId.replace(` `, `_`).replace(componentObject.id, ``)

            if (Object.getOwnPropertyDescriptor(componentObject, getterName)) { continue }
            Object.defineProperty(componentObject, getterName, {
                get: function() {
                    return Component.getObject(`${componentObjectId}${childComponentId}`)
                },
                set: function(newValue) {
                    console.error(`addChildComponentGettersToComponentObject: Cannot set ${getterName}.`)
                }
            })
        }
    }
    static loadIncludeComponent = async function (text, src, includeIn, componentClass, componentObjectId, include, keepTestingScript) {
        let fragment = ComponentLifecycle.compile(text)
        let fragmentAlreadyRegistered = window?.$components?.fragmentRegistry?.has(componentClass)
        let fragmentRegistered = fragmentAlreadyRegistered || ComponentLifecycle.registerDOMFragment(componentClass, fragment, keepTestingScript)

        if (!fragmentRegistered) {
            console.error(`loadIncludeComponent: Failed to register component fragment. Include processing halted. Component class: ${componentClass}. File containing bad Include-html tag is ${includeIn}. Include file is ${src}.`)
            return false
        }

        Loader.registerChildComponents(fragment, componentClass)

        let componentObject = ComponentLifecycle.createComponentObject(componentClass, componentObjectId, include)

        if (!componentObject) {
            console.error(`loadIncludeComponent: Failed to create component. Include processing halted. Component class: ${componentClass}. File containing bad Include-html tag is ${includeIn}. Include file is ${src}.`)
            return false
        }

        let componentObjectRegistered = ComponentLifecycle.registerComponentObject(componentClass, componentObjectId, componentObject, include)

        if (!componentObjectRegistered) {
            console.error(`loadIncludeComponent: Failed to register component object. Include processing halted. Component class: ${componentClass}. File containing bad Include-html tag is ${includeIn}. Include file is ${src}.`)
            return false
        }
        
        Loader.addChildComponentGettersToComponentObject(componentClass, componentObjectId)

        let componentMounted = ComponentLifecycle.mount(componentObjectId)

        if (!componentMounted) {
            console.error(`loadIncludeComponent: Failed to mount component object ${componentObjectId}. Include processing halted. File containing bad Include-html tag is ${includeIn}. Include file is ${src}.`)
            return false
        }

        return true
    }
    static loadIncludes = async function (keepTestingScript = false) {
        let includes = document.getElementsByTagName('include-html')

        if (0 === includes.length) { 
            Queue.broadcast(Loader.msgs.INCLUDES_LOADED, {})
            return 
        }
        for (let loop = 0; loop < includes.length; loop++) {
            const include = includes[loop]
            let result = await Loader.loadInclude(include, keepTestingScript)
            include.remove()
            if (!result) { 
                return 
            }
        }
        // Includes can contain includes.
        await Loader.loadIncludes()
    }
    static registerCustomTags = function () {
        // * The include-html tag can include a file containing HTML into another HTML document. The HTML to be
        // included can be a normal HTML snippet, or can be an HTML component. 
        // * In both cases the include-in attribute is used to indicate the name of the file receiving the HTML and 
        // the src attribute is used to provide the location of the file to be included.
        //
        // Including Components
        // * When including an HTML component, the include-html tag must also have a component-class attribute
        // indicating the class of the component and a component-id attribute providing an id for the component instance.
        // * An object of the type indicated by the component-class attrubute which represents the component instance is
        // created and can be accessed using the Component.getObject(id) method, where id is the value of the component-id
        // attribute of the include-html tag.
        // * The HTML DOM fragment used to create instance of the component can be accessed using the 
        // Component.getFragment(className) method, where className is the value of the component-class attribute of the
        // include-html tag.
        // * It is standard practice to give the root element of the component the same id as the component's class. This
        // makes it easy to locate the DOM element for the component.
        // The object represent an instance of a component has an id member, myComponent.id, that has the value provided by
        // the component-id attribute of the include-html.
        // The value of the id member of the object representing the component can be accessed in the HTML markup of the 
        // component in attributes or in innerText using the {id} syntax. Example: id="{id}".
        customElements.define('include-html', class IncludeHTMLElement extends HTMLElement { }, { })

        // * Components can have props, which are values that cannot be changed after the component has been initialized. The
        // values for the props of an instance of a component can be set using the include-props tag. The inner text of this
        // tag contans JSON used to inialize the props.
        // * The include-props tag can only appear inside the include-html tag of a component.
        // * The values of the props of the object representing the component can be accessed in the HTML markup of the 
        // component in attributes or in innerText using the {prop-name} syntax. Example: <div>{myProp}</div>".
        customElements.define('include-props', class IncludePropsElement extends HTMLElement { }, { })

        // * Components can have vars, which are values that can be changed throughout the lifetime of the component object. The
        // values for the vars of an instance of a component can be set using the include-vars tag. The inner text of this
        // tag contans JSON used to inialize the vars.
        // * The include-vars tag can only appear inside the include-html tag of a component. 
        // * The values of the vars of the object representing the component can be accessed in the HTML markup of the 
        // component in attributes or in innerText using the {var-name} syntax. Example: <div>{myVar}</div>".
        customElements.define('include-vars', class IncludeVarsElement extends HTMLElement { }, { })

        // * Defines a custom component. Custom components must contain:
        //      * A script tag which contains the javascript for a class that extends the Component class. The script
        //       tag may optionally contain additonal javascript.
        //      * A component-markup that provides the HTML markup for the component. The component-markup tag may optionally
        //       contain one or more component-slot tags.
        // * A custom-component tag may also contain:
        //      * A test-script tag that contains testing code for the tag.
        //      * A style tag that contains CSS. This is usually CSS used by the component.
        customElements.define('custom-component', class CustomComponentElement extends HTMLElement { }, { })

        // * Contains tests for the component. In normal web pages, this section will be removed from the DOM
        // when the tag is downloaded. On a testing page, the tests are executed.
        customElements.define('test-script', class TestScriptElement extends HTMLElement { }, { })

        // * A component-markup tag contains HTML markup that defines a component. The component-markup tag can
        // only appear inside a custom-component tag.
        customElements.define('component-markup', class ComponentMarkupElement extends HTMLElement { }, { })

        // * Slots are loaded after all incude tags have been processed.
        // * component-slot appears inside a component. It's only attribute is an id. It marks a location that can
        // accept content from a slot-markup tag.
        customElements.define('component-slot', class ComponentSlotElement extends HTMLElement { }, { })

        // * A slot-markup tag apears anywhere in the body of the page outside of a component. The slot-markup tag
        // has for-component-id and for-slot attributes that indicate which slot it's associated with. All children 
        // elements of the slot-markup tag become sibling elements of the associated component-slot tag. Once this is
        // done, the empty slot-markup and the component-slot tags are removed from the document.
        // * The contents of a slot-markup tag are associated only with the DOM elements of an instance of a component.
        // They are not associated with the DOM fragment.
        customElements.define('slot-markup', class SlotMarkupElement extends HTMLElement { }, { })
    }
}