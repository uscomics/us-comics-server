pub mod url {
    use std::collections::HashMap;

    // Functions for working with URLs and URL templates. A URL template is a URL that can contain params.
    // Example URL template: /user/:id
    // Example URL that matches template: /user/1234
    // URLs have scheme, authority, and possibly query fields. URL templates don't.
    // URL tempates possibly have param fields. URLs don't.
    pub fn has_scheme(url: & str) -> bool { url.to_string().contains(":") }
    pub fn get_scheme(url: & str) -> String {
        if !has_scheme(url) { return "".to_string(); }
        let mut url_iter = url.split(":");
        url_iter.next().unwrap().to_string()
    }
    pub fn remove_scheme(url: & str) -> String {
        if !has_scheme(url) { return url.to_string(); }
        let mut url_iter = url.split(":");
        url_iter.next();
        url_iter.next().unwrap().to_string()
    }
    pub fn has_authority (url: & str) -> bool { remove_scheme(url).to_string().contains("//") }
    pub fn get_authority (url: & str) -> String {
        if !has_authority(url) { return "".to_string(); }
        let mut url_clean = remove_scheme(url);
        url_clean = url_clean.trim_start_matches('/').to_string();
        let mut url_iter = url_clean.split("/");
        url_iter.next().unwrap().to_string()
    }
    pub fn remove_authority (url: & str) -> String {
        if !has_authority(url) { return url.to_string(); }
        let mut url_clean = remove_scheme(url);
        url_clean = url_clean.trim_start_matches('/').to_string();
        let mut url_iter = url_clean.split("/");
        url_iter.next();
        let mut result: String = "".to_owned();
        url_iter.for_each(|item| {
            result.push_str("/");
            result.push_str(item);
        });
        result.to_string()
    }
    pub fn has_params(template: & str) -> bool { template.contains(":") }
    pub fn get_params(url: & str, template: & str, params: &mut HashMap<String, String>) {
        let url_no_scheme = remove_scheme(url);
        let url_no_authority = remove_authority(url_no_scheme.as_str());
        let url_clean = remove_query(url_no_authority.as_str());
        let url_iter = url_clean.split("/");
        let template_iter = template.split("/");
        let zip_iter = template_iter.zip(url_iter);
        params.clear();
        zip_iter.for_each(|(template, url)| {
            let mut template_chars_iterator = template.chars();
            let first_char = template_chars_iterator.nth(0);
            match first_char {
                Some(c) => {
                    if ':' != c { return; }
                    let param_name: String = template_chars_iterator.collect();
                    params.insert(param_name, url.to_string());
                },
                None    => return
            }
        });
    }
    pub fn set_params(url: & str, template: & str) -> String { 
        let mut params: HashMap<String, String> = HashMap::new();
        get_params(url, template, &mut params);
        let params_iter = params.iter();
        let mut result = template.to_string();
        params_iter.for_each(|(key, value)| {
            let mut k: String = ":".to_owned();
            k.push_str(key);
            result = str::replace(result.as_str(), k.as_str(), value);
        });
        result
    }
    pub fn has_query(url: & str) -> bool { url.to_string().contains("?") }
    pub fn remove_query(url: & str) -> String {
        let mut url_iter = url.split("?");
        url_iter.next().unwrap().to_string()
    }
    pub fn get_query(url: & str, query: &mut HashMap<String, String>) {
        query.clear();
        if !has_query(url) { return; }
        let mut url_iter = url.split("?");
        url_iter.next();
        match url_iter.next() {
            Some(q) => {
                let query_text = q.to_string();
                let query_iter = query_text.split("&");
                query_iter.for_each(|query_item| {
                    let mut item_iter = query_item.split("=");
                    match item_iter.next() {
                        Some(key) => {
                            match item_iter.next() {
                                Some(value) => { query.insert(key.to_string(), value.to_string()); },
                                None    => return
                            }
                        },
                        None    => return
                    }
                })
            },
            None    => return
        }
    }
    pub fn matches(url: & str, template: & str) -> bool { 
        let url_clean = remove_query(remove_authority(remove_scheme(url).as_str()).as_str());
        let populated_template = set_params(url, template);
        url_clean == populated_template
    }
}

///////////////////////////////////////////
// Tests
// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod test {
    use super::url::*;
    use std::collections::HashMap;

    #[test]
    fn test_has_scheme() {
        assert_eq!(has_scheme("https://en.wikipedia.org/index"), true);
        assert_eq!(has_scheme("/index"), false);
    }

    #[test]
    fn test_get_scheme() {
        assert_eq!(get_scheme("https://en.wikipedia.org/index"), "https");
        assert_eq!(get_scheme("/index"), "");
    }

    #[test]
    fn test_remove_scheme() {
        assert_eq!(remove_scheme("https://en.wikipedia.org/index"), "//en.wikipedia.org/index");
        assert_eq!(remove_scheme("/index"), "/index");
    }

    #[test]
    fn test_has_authority() {
        assert_eq!(has_authority("https://en.wikipedia.org/index"), true);
        assert_eq!(has_authority("/index"), false);
    }

    #[test]
    fn test_get_authority() {
        assert_eq!(get_authority("https://en.wikipedia.org/index"), "en.wikipedia.org");
        assert_eq!(get_authority("/index"), "");
    }

    #[test]
    fn test_remove_authority() {
        assert_eq!(remove_authority("https://en.wikipedia.org/index"), "/index");
        assert_eq!(remove_authority("/index"), "/index");
    }

   #[test]
    fn test_has_params() {
        assert_eq!(has_params("/index"), false);
        assert_eq!(has_params("/index/:param"), true);
        assert_eq!(has_params("/index/?query=true"), false);
    }

    #[test]
    fn test_get_params() {
        let mut params: HashMap<String, String> = HashMap::new();
        get_params("/eat/at/joes", "/eat/at/joes", &mut params);
        assert_eq!(params.len(), 0);
        get_params("/eat/at/joes", "", &mut params);
        assert_eq!(params.len(), 0);
        get_params("", "/eat/at/joes", &mut params);
        assert_eq!(params.len(), 0);
        get_params("/eat/at/joes", "/:param1/:param2/:param3", &mut params);
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("param1").unwrap(), "eat");
        assert_eq!(params.get("param2").unwrap(), "at");
        assert_eq!(params.get("param3").unwrap(), "joes");
        get_params("/eat/at/joes", "/:param1/:param2/joes", &mut params);
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("param1").unwrap(), "eat");
        assert_eq!(params.get("param2").unwrap(), "at");

        get_params("https://en.wikipedia.org/eat/at/joes", "/eat/at/joes", &mut params);
        assert_eq!(params.len(), 0);
        get_params("https://en.wikipedia.org/eat/at/joes", "", &mut params);
        assert_eq!(params.len(), 0);
        get_params("", "/eat/at/joes", &mut params);
        assert_eq!(params.len(), 0);
        get_params("https://en.wikipedia.org/eat/at/joes", "/:param1/:param2/:param3", &mut params);
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("param1").unwrap(), "eat");
        assert_eq!(params.get("param2").unwrap(), "at");
        assert_eq!(params.get("param3").unwrap(), "joes");
        get_params("https://en.wikipedia.org/eat/at/joes", "/:param1/:param2/joes", &mut params);
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("param1").unwrap(), "eat");
        assert_eq!(params.get("param2").unwrap(), "at");
    }

    #[test]
    fn test_set_params() {
        let mut params_set = set_params("/eat/at/joes", "/eat/at/joes");
        assert_eq!(params_set, "/eat/at/joes");
        params_set = set_params("/eat/at/joes", "");
        assert_eq!(params_set, "");
        params_set = set_params("", "/:param1/:param2/:param3");
        assert_eq!(params_set, "/:param1/:param2/:param3");
        params_set = set_params("/eat/at/joes", "/:param1/:param2/:param3");
        assert_eq!(params_set, "/eat/at/joes");
        params_set = set_params("/eat/at/joes/now", "/:param1/:param2/:param3/now");
        assert_eq!(params_set, "/eat/at/joes/now");
    
        params_set = set_params("https://en.wikipedia.org/eat/at/joes", "/eat/at/joes");
        assert_eq!(params_set, "/eat/at/joes");
        params_set = set_params("https://en.wikipedia.org/eat/at/joes", "");
        assert_eq!(params_set, "");
        params_set = set_params("", "/:param1/:param2/:param3");
        assert_eq!(params_set, "/:param1/:param2/:param3");
        params_set = set_params("https://en.wikipedia.org/eat/at/joes", "/:param1/:param2/:param3");
        assert_eq!(params_set, "/eat/at/joes");
        params_set = set_params("https://en.wikipedia.org/eat/at/joes/now", "/:param1/:param2/:param3/now");
        assert_eq!(params_set, "/eat/at/joes/now");
    }

   #[test]
    fn test_has_query() {
        assert_eq!(has_query("/index"), false);
        assert_eq!(has_query("/index?query=true"), true);
        assert_eq!(has_query("/index/:param"), false);
    }

    #[test]
    fn test_qet_query() {
        let mut query: HashMap<String, String> = HashMap::new();
        get_query("/index", &mut query);
        assert_eq!(query.len(), 0);
        get_query("/index?title=Query_string", &mut query);
        assert_eq!(query.len(), 1);
        get_query("/index?title=Query_string&action=edit", &mut query);
        assert_eq!(query.len(), 2);
    }

    #[test]
    fn test_matches() {
        assert_eq!(matches("/index", "/index"), true);
        assert_eq!(matches("/user/1234", "/user/:id"), true);
        assert_eq!(matches("JUNK", "/index"), false);

        assert_eq!(matches("https://en.wikipedia.org/index", "/index"), true);
        assert_eq!(matches("https://en.wikipedia.org/user/1234", "/user/:id"), true);
        assert_eq!(matches("https://en.wikipedia.org/JUNK", "/index"), false);
    }
}