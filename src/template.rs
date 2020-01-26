// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

pub fn get_template(lang: String, name: String) -> Option<std::borrow::Cow<'static, [u8]>> {
    let pathname = format!("{}/{}.tpl", lang, name);
    Asset::get(pathname.as_str())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_template_test() {
        assert_ne!(None, get_template("fr".to_string(), "drop_comment".to_string()));
        assert_eq!(None, get_template("fr".to_string(), "unknown_template_name".to_string()));
    }
}