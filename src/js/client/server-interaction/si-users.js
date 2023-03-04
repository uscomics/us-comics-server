class SIUsers {
    static async login(userName, password) {
        const server = Registry.get(`Server`)
        const login = {name: userName, password}
        const response = await fetch(`${server}login-attempt`, {
            method: 'POST', 
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(login)
        })

        return response
    }
    static async get() {
        const server = Registry.get(`Server`)
        const response = await fetch(`${server}user-info`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            }
        })

        return response
    }
    static async add(userName, password, roles, title, firstName, lastName, file) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}

        const formData = new FormData()

        formData.append("userName", userName)
        formData.append("password", password)
        formData.append("roles", roles)
        formData.append("firstName", firstName)
        formData.append("lastName", lastName)
        formData.append("title", title)
        formData.append("filename", file)

        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}user-info`, {
                method: 'POST',
                headers: {
                    'Authorization': `'Bearer ${credentials.token}'`
                },
                body: formData
            })

            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
    static async update(userName, password, roles, title, firstName, lastName, file, id) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}


        const formData = new FormData()

        formData.append("userName", userName)
        formData.append("password", password)
        formData.append("roles", roles)
        formData.append("firstName", firstName)
        formData.append("lastName", lastName)
        formData.append("title", title)
        formData.append("filename", file)
        formData.append("id", id)

        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}user-info-update`, {
                method: 'POST',
                headers: {
                    'Authorization': `'Bearer ${credentials.token}'`
                    },
                body: formData
            })
    
            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
    static async remove(userId) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}

        const args = {
            method: 'DELETE',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `'Bearer ${credentials.token}'`
            },
            body: JSON.stringify({ id: userId })
        }

        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}user-info`, args)

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
}