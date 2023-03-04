module.exports.log = (entry) => {
    if (`NONE` === entry.log?.toUpperCase()) { return }

    const logLevel = process.env.LOG_LEVEL.toUpperCase()

    if (`INFO` === entry.log?.toUpperCase()) {
      if (`INFO` === logLevel || `WARN` === logLevel || `ERROR` === logLevel) { console.info(`${entry.verb} ${entry.path}`)}
    }
    else if (`WARN` === entry.log?.toUpperCase()) {
      if (`WARN` === logLevel || `ERROR` === logLevel) { console.warn(`${entry.verb} ${entry.path}`)}
    }
    else if (`ERROR` === entry.log?.toUpperCase()) {
      if (`ERROR` === logLevel) { console.error(`${entry.verb} ${entry.path}`)}
    }
}