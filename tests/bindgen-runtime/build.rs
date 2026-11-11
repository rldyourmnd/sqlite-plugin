fn main() {
    bindgen::Builder::default()
        .header_contents("runtime.h", "int runtime_test(void);")
        .generate()
        .expect("the second bindgen version must load libclang");
}
