// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn get_template(lang: String, name: String) -> Option<std::borrow::Cow<'static, [u8]>> {
    let pathname = format!("templates/{}/{}.tpl", lang, name);
    Asset::get(pathname.as_str())
}

