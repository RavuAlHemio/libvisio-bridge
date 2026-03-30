fn main() {
    println!("cargo::rerun-if-changed=src/libvisio-glue.cpp");
    println!("cargo::rerun-if-changed=src/libvisio-glue.h");

    let libvisio = pkg_config::Config::new()
        .probe("libvisio-0.1")
        .unwrap();
    let librevenge_stream = pkg_config::Config::new()
        .probe("librevenge-stream-0.0")
        .unwrap();

    cc::Build::new()
        .cpp(true)
        .file("src/libvisio-glue.cpp")
        .includes(&libvisio.include_paths)
        .includes(&librevenge_stream.include_paths)
        .compile("visio-glue");
}
