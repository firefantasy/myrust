fn main() {
    // 只有 build.rs 更改后才重新编译
    println!("cargo:rerun-if-changed=build.rs");
    pyo3_build_config::add_extension_module_link_args();
}