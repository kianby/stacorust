// --------------------------------------------------------------------------
//  stacorust.lib.template
// --------------------------------------------------------------------------

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Locale {
  FR,
  EN,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Template {
    lang: Locale,
    name: &'static str,
    content: &'static str
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}-{}={})", self.lang, self.name, self.content)
    }
}

impl Template {
    fn new(lang: Locale, name: &'static str, content: &'static str) -> Template {
       Template {lang: lang, name: name, content: content}
    }
}

fn build_key(t: Template) -> String {
    t.lang.to_string() + t.name
}

lazy_static! {
  static ref TEMPLATES: HashMap<String, Template> = {
    let mut dict = HashMap::new();
    let t1fr = build_key(Template::new(Locale::FR, "t1", "content1fr"));
    dict.insert(t1fr, Template::new(Locale::FR, "t1", "content1fr"));
    let t1en = build_key(Template::new(Locale::EN, "t1", "content1en"));
    dict.insert(t1en, Template::new(Locale::EN, "t1", "content1en"));
    dict
  };
}

pub fn find_template(lang: Locale, name: &'static str) -> Option<&Template> {
    let key : String = lang.to_string() + name;
    TEMPLATES.get(&key)
}
