const assert = (passed, description, results, optionalContinueOnError) => {
  if (description && results) { results.push({ description, passed: !!passed }) }
  if (!!passed || optionalContinueOnError) { return }
  if (results) { throw results } 
  else if (description) { throw new Error(description) }
  else { throw new Error(`Assert failed.`) }
}
const test = async (name, description, tests) => {
  let results = []
  let result = {
    name,
    description,
    passed: false,
    assertResults: [],
    duration: 0
  }
  try {
    for (let loop = 0; loop < tests.length; loop++) {
      let start = Date.now()
      let testResults = await tests[loop]()
      result.duration = Date.now() - start
      result.passed = true
      result.assertResults = result.assertResults.concat(testResults)
      results.push(result)
    }
  } catch (e) {
    if (Array.isArray(e)) {
      result.assertResults = e
    } else {
      result.assertResults.push(e.message)
    }
    results.push(result)
  }
  return results
}
const suite = async (name, description, suiteResults, optionalViewBuilder) => {
  let suiteTree = new Tree(name)
  let suiteFailCount = 0

  suiteTree.name = name
  suiteTree.description = description
  suiteTree.passed = true
  suiteTree.type = `SUITE`
  for (let loop1 = 0; loop1 < suiteResults.length; loop1++) {
    let testArray = suiteResults[loop1]

    for (let loop2 = 0; loop2 < testArray.length; loop2++) {
      let testResults = testArray[loop2]
      let testNode = new TreeNode(`Test-${loop2}`)
      let testFailCount = 0

      testNode.name = testResults.name
      testNode.description = testResults.description
      testNode.passed = true
      testNode.duration = testResults.duration
      testNode.type = `TEST`
      suiteTree.addNode(testNode)
      suiteTree.passed &&= testNode.passed
      for (let loop3 = 0; loop3 < testResults.assertResults.length; loop3++) {
        let assertResult = testResults.assertResults[loop3]
        let assertionNode = testNode.addChild(`Assertion-${loop3}`)

        assertionNode.description = assertResult.description
        assertionNode.passed = assertResult.passed
        assertionNode.type = `ASSERTION`
        testNode.passed &&= assertionNode.passed
        suiteTree.passed &&= assertionNode.passed
        if (!assertionNode.passed) { 
          suiteFailCount++
          testFailCount++
        }
      }
      testNode.failCount = testFailCount
    }
    suiteTree.failCount = suiteFailCount
  }
  optionalViewBuilder && optionalViewBuilder(suiteTree)
  return suiteTree
}
/*
const suite = async (name, description, testResults) => {
  let suitePassed = true

  for (let loop = 0; loop < testResults.length; loop++) {
    let testResultArray = testResults[loop]
    for (let loop2 = 0; loop2 < testResultArray.length; loop2++) {
      let testResult = testResultArray[loop2]
      if (!testResult.passed) {
        suitePassed = false
        break
      }
    }
  }
  let cardSuiteDiv = _createCardSuiteDiv(document.getElementById(`CardTestingResults`), suitePassed, name, description)
  let textSuiteDiv = _createTextSuiteDiv(document.getElementById(`TextTestingResults`), suitePassed, name, description)

  for (let loop = 0; loop < testResults.length; loop++) {
    let results = testResults[loop]
    for (let loop2 = 0; loop2 < results.length; loop2++) {
      let testTextColor = results[loop2].passed? `success` : `error`
      let assertResults = results[loop2].assertResults
      let cardTestWrapperDiv = _createCardTestWrapperDiv(cardSuiteDiv)
      let textTestWrapperDiv = _createTextTestWrapperDiv(textSuiteDiv)
      
      _createCardTestNameDiv(cardTestWrapperDiv)
      let cardTestPassedDiv = _createCardTestPassedDiv(cardTestWrapperDiv)
      _createCardTestDurationDiv(cardTestPassedDiv)

      _createTextTestNameDiv(textTestWrapperDiv)
      _createTextTestDescriptionDiv(textTestWrapperDiv)
      _createTextTestDurationDiv(textTestWrapperDiv)
      _createTextTestPassedDiv(textTestWrapperDiv)
      for (let loop = 0; loop < assertResults.length; loop++) {
        const _createTextTestAssertResultDiv = (suiteDiv, assertResult, assertCount) => {
          let testAssertResultDiv = document.createElement('div')
          let assertColor = (loop < assertResults.length - 1)? `success` : testTextColor

          testAssertResultDiv.id = `TextTestPassedAssertResul${results[loop2].name}-${assertCount}`
          testAssertResultDiv.className = `caption-2 margin-l-20 ${assertColor}`
          testAssertResultDiv.innerText = assertResult
          suiteDiv.appendChild(testAssertResultDiv)
        }
        let assertResult = assertResults[loop]
        _createTextTestAssertResultDiv(textSuiteDiv, assertResult, loop)
      }
    }
  }
}
*/

const _createTextSuiteDiv = (rootElement, passed, name, description) => {
  let suiteDiv = document.createElement('div')
  let suiteText = `Suite Detail: ${name} ${description}`
  let suiteTextColor = passed? `success` : `error`
  suiteDiv.id = `TextSuite${name}`
  suiteDiv.className = `flex-col caption-1 margin-5 p5 ${suiteTextColor}`
  suiteDiv.innerText = suiteText
  rootElement.appendChild(suiteDiv)
  return suiteDiv
}
const _createTextTestWrapperDiv = (suiteDiv) => {
  let testWrapperDiv = document.createElement('div')
  testWrapperDiv.id = `TextTestWrapper${results[loop2].name}`
  testWrapperDiv.className = `flex-row caption-2 ${testTextColor}`
  suiteDiv.appendChild(testWrapperDiv)
  return testWrapperDiv
}
const _createTextTestNameDiv = (testWrapperDiv) => {
  let testNameDiv = document.createElement('div')
  testNameDiv.id = `TextTestName${results[loop2].name}`
  testNameDiv.className = `margin-l-10 ${testTextColor}`
  testNameDiv.innerText = `Test ${results[loop2].name}`
  testWrapperDiv.appendChild(testNameDiv)
}
const _createTextTestDescriptionDiv = (testWrapperDiv) => {
  let testDescriptionDiv = document.createElement('div')
  testDescriptionDiv.id = `TextTestDescription${results[loop2].name}`
  testDescriptionDiv.className = `margin-l-10 ${testTextColor}`
  testDescriptionDiv.innerText = results[loop2].description
  testWrapperDiv.appendChild(testDescriptionDiv)
}
const _createTextTestDurationDiv = (testWrapperDiv) => {
  let testDurationnDiv = document.createElement('div')
  testDurationnDiv.id = `TextTestDuration${results[loop2].name}`
  testDurationnDiv.className = `margin-l-10 ${testTextColor}`
  testDurationnDiv.innerText = `` + results[loop2].duration + "ms"
  testWrapperDiv.appendChild(testDurationnDiv)
}
const _createTextTestPassedDiv = (testWrapperDiv) => {
  let testPassedDiv = document.createElement('div')
  testPassedDiv.id = `TextTestPassed${results[loop2].name}`
  testPassedDiv.className = `margin-l-10 ${testTextColor}`
  testPassedDiv.innerText = results[loop2].passed? "Passed" : "Failed"
  testWrapperDiv.appendChild(testPassedDiv)
}
const _createCardSuiteDiv = (rootElement, passed, name, description) => {
  let suiteDiv = document.createElement('div')
  let suiteDivName = document.createElement('div')
  let suiteDivDescription = document.createElement('div')
  let suiteDivTests = document.createElement('div')
  let suiteTextColor = passed? `success` : `error`
  let emoji = passed? `👍` : `👎`
  suiteDiv.id = `CardSuite${name}`
  suiteDiv.className = `flex-col flex-space-around w95 margin-lr-5 margin-b-5 p5 border-3 border-solid border-black ${suiteTextColor}`
  suiteDivName.id = `CardSuiteName${name}`
  suiteDivName.className = `heading-6 ${suiteTextColor}`
  suiteDivName.innerText = `Suite Summary: ${name} ${emoji}`
  suiteDivDescription.id = `CardSuiteDescription${name}`
  suiteDivDescription.className = `caption-1 margin-b-5 ${suiteTextColor}`
  suiteDivDescription.innerText = description
  suiteDivTests.id = `CardSuiteTestsRow${name}`
  suiteDivTests.className = `flex-row flex-wrap flex-space-between margin-lr-5`
  suiteDiv.appendChild(suiteDivName);
  suiteDiv.appendChild(suiteDivDescription);
  suiteDiv.appendChild(suiteDivTests);
  rootElement.appendChild(suiteDiv)
  return suiteDivTests
}
const _createCardTestWrapperDiv = (suiteDiv) => {
  let testWrapperDiv = document.createElement('div')
  testWrapperDiv.id = `CardTestWrapper${results[loop2].name}`
  testWrapperDiv.className = `flex-col center-text caption-2 margin-2 w160px h100 border-1 border-solid border-black ${testTextColor}`
  suiteDiv.appendChild(testWrapperDiv)
  return testWrapperDiv
}
const _createCardTestNameDiv = (testWrapperDiv) => {
  let testNameDiv = document.createElement('div')
  testNameDiv.id = `CardTestName${results[loop2].name}`
  testNameDiv.className = `w100 ${testTextColor}`
  testNameDiv.innerText = `Test ${results[loop2].name}`
  testWrapperDiv.appendChild(testNameDiv)
}
const _createCardTestDurationDiv = (testWrapperDiv) => {
  let testDurationnDiv = document.createElement('div')
  testDurationnDiv.id = `CardTestDuration${results[loop2].name}`
  testDurationnDiv.className = `caption-2 ${testTextColor}`
  testDurationnDiv.innerText = `` + results[loop2].duration + "ms"
  testWrapperDiv.appendChild(testDurationnDiv)
}
const _createCardTestPassedDiv = (testWrapperDiv) => {
  let testPassedRowDiv = document.createElement('div')
  let testPassedDiv = document.createElement('div')
  testPassedRowDiv.id = `CardTestPassedROw${results[loop2].name}`
  testPassedRowDiv.className = `flex-row flex-space-around w100`
  testPassedDiv.id = `CardTestPassed${results[loop2].name}`
  testPassedDiv.className = `flex-row caption-2 margin-lr-5`
  testPassedDiv.innerText = results[loop2].passed? `👍` : `👎`
  testWrapperDiv.appendChild(testPassedRowDiv)
  testPassedRowDiv.appendChild(testPassedDiv)
  return testPassedRowDiv
}