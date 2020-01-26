// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

fn fill_template(template: String, _params: &HashMap<String, String>) -> String {
    // TODO replace params {{ param 1 }}
    template
}

pub fn get_template(
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
}
