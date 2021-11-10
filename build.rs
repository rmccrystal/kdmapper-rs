use std::fs;
use std::path::Path;

fn main() {
    // get all cpp files in the kdmapper/kdmapper dir
    let files = fs::read_dir("kdmapper/kdmapper").unwrap()
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .filter(|p| p.extension().map(|n| n.to_str()) == Some(Some("cpp")));


    let mut build = cc::Build::new();
    build
        .files(files)
        .file("src/interop.cpp")

        .include("kdmapper/kdmapper")
        .include("atlmfc/include")

        .cpp(true)
        .flag("/std:c++17");

    if cfg!(feature = "disable-output") {
        build.define("DISABLE_OUTPUT", None);
    }

    build.compile("kdmapper");

    println!("cargo:rustc-link-search=native={}", Path::new("atlmfc/lib/x64").display());
}