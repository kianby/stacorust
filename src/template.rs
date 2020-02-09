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

fn fill_template(template: String, params: &HashMap<&str, &str>) -> String {
    let mut content = template;
    for (name, value) in params {
        let pattern = format!("{{{{ {} }}}}", name);
        content = content.replace(&pattern, value);
    }
    content
}

fn get_template(lang: &Lang, name: &str, params: &HashMap<&str, &str>) -> Option<String> {
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

pub fn get_template_approve_comment(lang: &Lang, original: &str) -> Option<String> {
    let params: HashMap<&str, &str> = [("original", original)].iter().cloned().collect();
    get_template(lang, "approve_comment", &params)
}

pub fn get_template_drop_comment(lang: &Lang, original: &str) -> Option<String> {
    let params: HashMap<&str, &str> = [("original", original)].iter().cloned().collect();
    get_template(lang, "drop_comment", &params)
}

pub fn get_template_new_comment(lang: &Lang, url: &str, comment: &str) -> Option<String> {
    let params: HashMap<&str, &str> = [("url", url), ("comment", comment)]
        .iter()
        .cloned()
        .collect();
    get_template(lang, "new_comment", &params)
}

pub fn get_template_notify_message(lang: &Lang) -> Option<String> {
    let params: HashMap<&str, &str> = HashMap::new();
    get_template(lang, "notify_message", &params)
}

pub fn get_template_rss_title_message(lang: &Lang, site: &str) -> Option<String> {
    let params: HashMap<&str, &str> = [("site", site)].iter().cloned().collect();
    get_template(lang, "rss_title_message", &params)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_template_test() {
        let noparams: HashMap<&str, &str> = HashMap::new();
        assert_ne!(None, get_template(&Lang::Fr, "drop_comment", &noparams));
        assert_eq!(
            None,
            get_template(&Lang::En, "unknown_template_name", &noparams)
        );
    }

    #[test]
    fn fill_template_test() {
        let mut p: HashMap<&str, &str> = HashMap::new();
        p.insert("var", "toto");
        p.insert("count", "25");
        assert_eq!(
            "test=toto, count=25",
            fill_template("test={{ var }}, count={{ count }}".to_string(), &p)
        )
    }

    #[test]
    fn exist_template_test() {
        assert_ne!(None, get_template_approve_comment(&Lang::Fr, ""));
        assert_ne!(None, get_template_drop_comment(&Lang::Fr, ""));
        assert_ne!(None, get_template_new_comment(&Lang::Fr, "", ""));
        assert_ne!(None, get_template_notify_message(&Lang::Fr));
        assert_ne!(None, get_template_rss_title_message(&Lang::Fr, ""));
    }
}
