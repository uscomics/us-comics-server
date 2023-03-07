class SIImgurAlbum {
    static async get() {
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}albums`, {
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
    static async add(title, optionalDescription) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const formData = new FormData()
    
            formData.append("title", title)
            if (optionalDescription) { formData.append("description", optionalDescription) }
    
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}album`, {
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
    static async remove(albumId) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const args = {
                method: 'DELETE',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': `'Bearer ${credentials.token}'`
                },
                body: JSON.stringify({ })
            }

            const server = Registry.get(`Server`)
            const response = await fetch(`${server}/album/${id}`, args)

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
}