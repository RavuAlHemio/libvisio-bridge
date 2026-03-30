fn main() {
    println!("cargo::rerun-if-changed=src/libvisio-glue.cpp");
    println!("cargo::rerun-if-changed=src/libvisio-glue.h");

    let deps = pkg_config::Config::new()
        .probe("libvisio-0.1")
        .unwrap();

    cc::Build::new()
        .cpp(true)
        .file("src/libvisio-glue.cpp")
        .includes(&deps.include_paths)
        .compile("visio-glue");
}
