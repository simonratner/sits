use std::env;
use std::fs;
use std::path::Path;

extern crate toml;
extern crate mustache;
extern crate rustc_serialize;

fn getenv(var: &str) -> String {
    env::var(var).unwrap()
}

#[cfg(target_os = "windows")]
fn generate_manifest() {

    #[derive(Debug, RustcEncodable)]
    struct CargoData<'a> {
        name: &'a str,
        version: &'a str,
        description: &'a str,
    };

    println!("     Parsing cargo manifest");
    let cargo = toml::Value::Table(
        toml::Parser::new(include_str!("Cargo.toml")).parse().unwrap()
    );
    let name = cargo.lookup("package.name").unwrap().as_str().unwrap();
    let version = cargo.lookup("package.version").unwrap().as_str().unwrap();
    let description = match cargo.lookup("package.description") {
        Some(&toml::Value::String(ref v)) => v.as_ref(),
        _ => name,
    };
    let cargo_data = CargoData {
        name: name,
        version: version,
        description: description,
    };

    let profile = getenv("PROFILE");
    let src = Path::new("application.manifest");
    let dest = Path::new("target").join(&profile).join(name).with_extension("exe.manifest");

    println!("     Parsing {:?}", src);
    let manifest = mustache::compile_path(src).unwrap();

    println!("     Rendering {:?} with:\n       {:?}", dest, cargo_data);
    let mut f = fs::File::create(&dest).unwrap();
    manifest.render(&mut f, &cargo_data).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn generate_manifest() {}

fn main() {
    generate_manifest();
}
