class SIImgurImage {
    static async get(albumId) {
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}album/${albumId}/images`, {
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
    static async getOne(imageId) {
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}image/${imageId}`, {
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
    static async add(albumId, title, file, optionalDescription) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const formData = new FormData()
    
            formData.append("albumId", albumId)
            formData.append("title", title)
            formData.append("filename", file)
            if (optionalDescription) { formData.append("description", optionalDescription) }
    
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}image`, {
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
            const response = await fetch(`${server}/image/${id}`, args)

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
}