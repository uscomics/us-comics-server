class SIText {
    static async get() {
        const server = Registry.get(`Server`)
        const response = await fetch(`${server}text-info`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            }
        })

        return response
    }
    static async update(text, id) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}

        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}text-info-update`, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': `'Bearer ${credentials.token}'`
                },
                body: JSON.stringify({ text, id })
            })
    
            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
}