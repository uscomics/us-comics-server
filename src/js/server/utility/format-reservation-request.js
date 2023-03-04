module.exports = (json) => {
    const err = `400 Bad Request`
    let text = ``

    if (!json) {
        console.error(err + `: No json provided.`)
        return { status: 400, text: err }
    }
    if (!json.firstName || !json.lastName) {
        console.error(err + `: Must provide firstName and lastName.`)
        return { status: 400, text: err }
    }
    text += `Name: ${json.firstName} ${json.lastName}\n`
    if (!json.mobileNumber) {
        console.error(err + `: Must provide mobileNumber.`)
        return { status: 400, text: err }
    }
    text += `Phone: ${json.mobileNumber}\n`
    if (!json.pickupLocation) {
        console.error(err + `: Must provide pickupLocation.`)
        return { status: 400, text: err }
    }
    if (!json.pickupLocation.address && !json.pickupLocation.airport) {
        console.error(err + `: Must provide address or airline information.`)
        return { status: 400, text: err }
    }
    text += `\nPickup Information\n`
    if (json.pickupLocation.address) {
        text += `Address: ${json.pickupLocation.address}\n`
        if (json.pickupLocation.address2) { text += `Address2: ${json.pickupLocation.address2}\n` }
        if (!json.pickupLocation.city) {
            console.error(err + `: Must provide city information.`)
            return { status: 400, text: err }
        }
        text += `City: ${json.pickupLocation.city}\n`
        if (!json.pickupLocation.state) {
            console.error(err + `: Must provide state information.`)
            return { status: 400, text: err }
        }
        text += `State: ${json.pickupLocation.state}\n`
        if (!json.pickupLocation.date) {
            console.error(err + `: Must provide date information.`)
            return { status: 400, text: err }
        }
        text += `Date: ${json.pickupLocation.date}\n`
        if (!json.pickupLocation.time) {
            console.error(err + `: Must provide time information.`)
            return { status: 400, text: err }
        }
        text += `Time: ${json.pickupLocation.time}\n`
    } else {
        text += `Airline: ${json.pickupLocation.airline}\n`
        if (!json.pickupLocation.flightNumber) {
            console.error(err + `: Must provide flightNumber information.`)
            return { status: 400, text: err }
        }
        text += `Flight Number: ${json.pickupLocation.flightNumber}\n`
        if (!json.pickupLocation.arrivalDate) {
            console.error(err + `: Must provide arrivalDate information.`)
            return { status: 400, text: err }
        }
        text += `Arrival Date: ${json.pickupLocation.arrivalDate}\n`
        if (!json.pickupLocation.estimatedArrivalTime) {
            console.error(err + `: Must provide estimatedArrivalTime information.`)
            return { status: 400, text: err }
        }
        text += `Estimated Arrival Time: ${json.pickupLocation.estimatedArrivalTime}\n`
    }
    if (!json.dropoffLocation) {
        console.error(err + `: Must provide dropoffLocation.`)
        return { status: 400, text: err }
    }
    text += `\nDropoff Information\n`
    if (json.dropoffLocation.address) {
        text += `Address: ${json.dropoffLocation.address}\n`
        if (json.dropoffLocation.address2) { text += `Address2: ${json.dropoffLocation.address2}\n` }
        if (!json.dropoffLocation.city) {
            console.error(err + `: Must provide city information.`)
            return { status: 400, text: err }
        }
        text += `City: ${json.dropoffLocation.city}\n`
        if (!json.dropoffLocation.state) {
            console.error(err + `: Must provide state information.`)
            return { status: 400, text: err }
        }
        text += `State: ${json.dropoffLocation.state}\n`
        /*
        if (!json.dropoffLocation.date) {
            console.error(err + `: Must provide date information.`)
            return { status: 400, text: err }
        }
        text += `Date: ${json.dropoffLocation.date}\n`
        if (!json.dropoffLocation.time) {
            console.error(err + `: Must provide time information.`)
            return { status: 400, text: err }
        }
        text += `Time: ${json.dropoffLocation.time}\n`
        */
    } else {
        text += `Airline: ${json.dropoffLocation.airline}\n`
        /*
        if (!json.dropoffLocation.flightNumber) {
            console.error(err + `: Must provide flightNumber information.`)
            return { status: 400, text: err }
        }
        text += `Flight Number: ${json.dropoffLocation.flightNumber}\n`
        if (!json.dropoffLocation.arrivalDate) {
            console.error(err + `: Must provide arrivalDate information.`)
            return { status: 400, text: err }
        }
        text += `Arrival Date: ${json.dropoffLocation.arrivalDate}\n`
        if (!json.dropoffLocation.estimatedArrivalTime) {
            console.error(err + `: Must provide estimatedArrivalTime information.`)
            return { status: 400, text: err }
        }
        text += `Estimated Arrival Time: ${json.dropoffLocation.estimatedArrivalTime}\n`
        */
    }
    if (json.specialInstructions) { text += `\nSpecial Instructions: ${json.specialInstructions}\n`}

    return { status: 200, text }
}