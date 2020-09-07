fn main() {
    /*
    let cwd = env::current_dir().unwrap();
    let cwd_str = cwd.to_str().unwrap();
    println!("cargo:rustc-link-search=native={}\\cpp_src\\TryCatchMemcpy\\x64\\Release", &cwd_str);
    println!("cargo:rustc-link-lib=static=TryCatchMemcpy");
    */
    let mut cfg = cc::Build::new();
    cfg.file("src/windows/TryCatchMemcpy.cpp");
    cfg.compile("TryCatchMemcpy");
    println!("cargo:rerun-if-changed=cpp_src/TryCatchMemcpy/TryCatchMemcpy.cpp");
}
