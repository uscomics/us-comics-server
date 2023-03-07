class SINews {
    static async get() {
        try {
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}character`, {
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
    static async add(file, title) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const formData = new FormData()
            const server = Registry.get(`Server`)
    
            formData.append(`filename`, file)
            formData.append(`title`, title)
            
            const response = await fetch(`${server}character`, {
                method: 'POST',
                headers: { 'Authorization': `'Bearer ${credentials.token}'` },
                body: formData
            })
    
            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
    static async update(file, title, text, id) {
        try {
            const credentials = JavascriptWebToken.getCredentials()

            if (!JavascriptWebToken.areCredentialsValid(credentials)) { return { status: 401 }}
    
            const formData = new FormData()
    
            formData.append("filename", file)
            formData.append("title", title)
            formData.append("text", text)
            formData.append("id", id)
            
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}character`, {
                method: 'PUT',
                headers: { 'Authorization': `'Bearer ${credentials.token}'` },
                body: formData
            })
    
            return response    
        } catch (e) {
            return { status: 401 }
        }
    }
    static async remove(newsId) {
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
                body: JSON.stringify({ id: newsId })
            }
    
            const server = Registry.get(`Server`)
            const response = await fetch(`${server}character`, args)

            return response
        } catch (e) {
            return { status: 401 }
        }
    }
}