// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

fn fill_template(template: String, params: &HashMap<String, String>) -> String {
    let mut content = template;
    for (name, value) in params {
        let pattern = format!("{{{{ {} }}}}", name);
        content = content.replace(&pattern, value);
    }
    content
}

fn get_template(
    lang: String,
    name: String,
    params: &HashMap<String, String>,
) -> Option<String> {
    let pathname = format!("{}/{}.tpl", lang, name);
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

pub fn get_template_approve_comment(lang: String, original: String) -> Option<String> {
    let params : HashMap<String, String> = [("original".to_string(), original)].iter().cloned().collect();
    get_template(lang, "approve_comment".to_string(), &params)
}

pub fn get_template_drop_comment(lang: String, original: String) -> Option<String> {
    let params : HashMap<String, String> = [("original".to_string(), original)].iter().cloned().collect();
    get_template(lang, "drop_comment".to_string(), &params)
}

pub fn get_template_new_comment(lang: String, url: String, comment: String) -> Option<String> {
    let params : HashMap<String, String> = [("url".to_string(), url), ("comment".to_string(), comment)].iter().cloned().collect();
    get_template(lang, "new_comment".to_string(), &params)
}

pub fn get_template_notify_message(lang: String) -> Option<String> {
    let params : HashMap<String, String> = HashMap::new();
    get_template(lang, "notify_message".to_string(), &params)
}

pub fn get_template_rss_title_message(lang: String, site: String) -> Option<String> {
    let params : HashMap<String, String> = [("site".to_string(), site)].iter().cloned().collect();
    get_template(lang, "rss_title_message".to_string(), &params)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_template_test() {
        let noparams: HashMap<String, String> = HashMap::new();
        assert_ne!(
            None,
            get_template("fr".to_string(), "drop_comment".to_string(), &noparams)
        );
        assert_eq!(
            None,
            get_template(
                "fr".to_string(),
                "unknown_template_name".to_string(),
                &noparams
            )
        );
    }

    #[test]
    fn fill_template_test() {
        let mut p = HashMap::new();
        p.insert("var".to_string(), "toto".to_string());
        assert_eq!("test=toto", fill_template("test={{ var }}".to_string(), &p))
    }

    #[test]
    fn exist_template_test() {
        assert_ne!(
            None,
            get_template_approve_comment("fr".to_string(), "".to_string())
        );
        assert_ne!(
            None,
            get_template_drop_comment("fr".to_string(), "".to_string())
        );  
        assert_ne!(
            None,
            get_template_new_comment("fr".to_string(), "".to_string(), "".to_string())
        );     
        assert_ne!(
            None,
            get_template_notify_message("fr".to_string())
        );       
        assert_ne!(
            None,
            get_template_rss_title_message("fr".to_string(), "".to_string())
        );       
    }
}
