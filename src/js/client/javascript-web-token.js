class JavascriptWebToken {
    static credentials = ``
    static getCredentials() { return JavascriptWebToken.credentials }
    static storeCredentials(token) { JavascriptWebToken.credentials = token }
    static areCredentialsValid(credentials) { return !!credentials?.roles?.length }
    static parseJWT (token) {
        var base64Url = token.split('.')[1];
        var base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
        var jsonPayload = decodeURIComponent(window.atob(base64).split('').map(function(c) {
            return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
        }).join(''));

        return JSON.parse(jsonPayload);
    }
    static deleteTokenDatabase() {
        JavascriptWebToken.credentials = null
    }
}