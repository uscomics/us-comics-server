class SIPhotos {
    static async get() {
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}photo-info`, {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                }
            })

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
    static async add(file, text) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const formData = new FormData()
    
            formData.append("filename", file)
            formData.append("text", text)
    
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}photo-info`, {
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
    static async update(file, text, id) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}

        const formData = new FormData()

        formData.append("filename", file)
        formData.append("text", text)
        formData.append("id", id)
        
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}photo-info-update`, {
                method: 'POST',
                headers: { 'Authorization': `'Bearer ${credentials.token}'` },
                body: formData
            })
    
            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
    static async remove(photoId) {
        const credentials = JavascriptWebToken.getCredentials()

        if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}

        const args = {
            method: 'DELETE',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `'Bearer ${credentials.token}'`
            },
            body: JSON.stringify({ id: photoId })
        }

        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}photo-info`, args)

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
}