fn main() {
    bindgen::Builder::default()
        .header_contents("runtime.h", "typedef unsigned int RuntimeTest;")
        .generate()
        .expect("the second bindgen version must load libclang")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("runtime.rs"),
        )
        .expect("write the generated runtime test type");
}
