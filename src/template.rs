// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Lang {
    En,
    Fr,
}

impl Lang {
    pub fn new(value: String) -> Option<Self> {
        let langs = vec![Lang::En, Lang::Fr];
        for lang in langs {
            if lang.to_string().to_lowercase() == value.to_lowercase() {
                return Some(lang);
            }
        }
        None
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

fn fill_template(template: String, params: &HashMap<String, &String>) -> String {
    let mut content = template;
    for (name, value) in params {
        let pattern = format!("{{{{ {} }}}}", name);
        content = content.replace(&pattern, value);
    }
    content
}

fn get_template(lang: &Lang, name: String, params: &HashMap<String, &String>) -> Option<String> {
    let pathname = format!("{}/{}.tpl", lang.to_string().to_lowercase(), name);
    let something = Asset::get(pathname.as_str());
    match something {
        None => None,
        Some(template) => {
            let template = String::from_utf8(template.as_ref().to_vec());
            match template {
                Ok(tpl) => Some(fill_template(tpl, &params)),
                Err(_error) => None,
            }
        }
    }
}

pub fn get_template_approve_comment(lang: &Lang, original: &String) -> Option<String> {
    let params: HashMap<String, &String> = [("original".to_string(), original)]
        .iter()
        .cloned()
        .collect();
    get_template(lang, "approve_comment".to_string(), &params)
}

pub fn get_template_drop_comment(lang: &Lang, original: &String) -> Option<String> {
    let params: HashMap<String, &String> = [("original".to_string(), original)]
        .iter()
        .cloned()
        .collect();
    get_template(lang, "drop_comment".to_string(), &params)
}

pub fn get_template_new_comment(lang: &Lang, url: &String, comment: &String) -> Option<String> {
    let params: HashMap<String, &String> =
        [("url".to_string(), url), ("comment".to_string(), comment)]
            .iter()
            .cloned()
            .collect();
    get_template(lang, "new_comment".to_string(), &params)
}

pub fn get_template_notify_message(lang: &Lang) -> Option<String> {
    let params: HashMap<String, &String> = HashMap::new();
    get_template(lang, "notify_message".to_string(), &params)
}

pub fn get_template_rss_title_message(lang: &Lang, site: &String) -> Option<String> {
    let params: HashMap<String, &String> = [("site".to_string(), site)].iter().cloned().collect();
    get_template(lang, "rss_title_message".to_string(), &params)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_template_test() {
        let noparams: HashMap<String, &String> = HashMap::new();
        assert_ne!(
            None,
            get_template(&Lang::Fr, "drop_comment".to_string(), &noparams)
        );
        assert_eq!(
            None,
            get_template(&Lang::En, "unknown_template_name".to_string(), &noparams)
        );
    }

    #[test]
    fn fill_template_test() {
        let mut p: HashMap<String, &String> = HashMap::new();
        let var_value = "toto".to_string();
        let count_value = "25".to_string();
        p.insert("var".to_string(), &var_value);
        p.insert("count".to_string(), &count_value);
        assert_eq!(
            "test=toto, count=25",
            fill_template("test={{ var }}, count={{ count }}".to_string(), &p)
        )
    }

    #[test]
    fn exist_template_test() {
        assert_ne!(
            None,
            get_template_approve_comment(&Lang::Fr, &"".to_string())
        );
        assert_ne!(None, get_template_drop_comment(&Lang::Fr, &"".to_string()));
        assert_ne!(
            None,
            get_template_new_comment(&Lang::Fr, &"".to_string(), &"".to_string())
        );
        assert_ne!(None, get_template_notify_message(&Lang::Fr));
        assert_ne!(
            None,
            get_template_rss_title_message(&Lang::Fr, &"".to_string())
        );
    }
}
