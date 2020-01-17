use ini::Ini;

fn main() {
    let i = Ini::load_from_file("config.ini").unwrap();
    for (sec, prop) in i.iter() {
        println!("Section: {:?}", *sec);
        for (k, v) in prop.iter() {
            println!("  {}:{}", *k, *v);
        }
    }
}
