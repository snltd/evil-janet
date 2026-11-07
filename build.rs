fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let illumos = target_os == "illumos" || target_os == "solaris";

    let mut clang_args: Vec<&str> = vec![];
    if illumos {
        clang_args.extend(["-D__EXTENSIONS__", "-DJANET_NO_NANBOX"]);
    }

    // Cause compilation error when both almagation and link-system is set
    #[cfg(all(feature = "link-amalg", feature = "link-system"))]
    compile_error!(r#"You can only use either "link-amalg" or "system" feature, not both."#);

    // Cause compilation error when both almagation and system is set
    #[cfg(all(feature = "link-amalg", feature = "system"))]
    compile_error!(r#"You can only use either "link-amalg" or "system" feature, not both."#);

    // Link to the system libjanet if the system feature is included
    #[cfg(any(feature = "system", feature = "link-system"))]
    println!("cargo:rustc-link-lib=janet");

    // Make cargo rerun if header changes
    println!("cargo:rerun-if-changed=csrc/janet.h");
    println!("cargo:rerun-if-changed=csrc/non_nanbox_shims.c");

    let allowlist_regex = "^[jJ]anet.*|[jJ]ANET.*|.*jmp";

    #[cfg(any(feature = "system", feature = "link-system"))]
    let header = std::env::var("JANET_HEADERPATH")
        .expect("JANET_HEADERPATH not found, maybe you don't have Janet installed.")
        + "/janet.h";

    #[cfg(not(any(feature = "system", feature = "link-system")))]
    let header = "./csrc/janet.h";

    let bindings = bindgen::Builder::default()
        .header(header)
        .header("./csrc/config_bits.h")
        .clang_args(clang_args.iter())
        .blocklist_var("JANET_CURRENT_CONFIG_BITS")
        .derive_debug(true)
        .use_core()
        .ctypes_prefix("::core::ffi")
        .generate_cstr(true)
        .allowlist_type(allowlist_regex)
        .allowlist_function(allowlist_regex)
        .allowlist_var(allowlist_regex)
        .override_abi(bindgen::Abi::CUnwind, allowlist_regex)
        .rust_target(bindgen::RustTarget::stable(77, 0).expect("Missing rust target"))
        .formatter(bindgen::Formatter::Rustfmt);

    let bindings = bindings.generate().expect("Unable to generate bindings");

    // Redefine some Janet configs using environment variables
    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    let recursion_guard = option_env!("JANET_RECURSION_GUARD");
    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    let max_proto_depth = option_env!("JANET_MAX_PROTO_DEPTH");
    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    let max_macro_expand = option_env!("JANET_MAX_MACRO_EXPAND");
    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    let max_stack = option_env!("JANET_STACK_MAX");

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    let mut build = cc::Build::new();

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if illumos {
        build.define("__EXTENSIONS__", None);
        build.define("JANET_NO_NANBOX", None);
        println!("cargo:rustc-link-lib=socket");
    }

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    build.file("csrc/janet.c").include("csrc");
    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    build.file("csrc/non_nanbox_shims.c");

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if let Some(val) = recursion_guard {
        build.define("JANET_RECURSION_GUARD", val);
    }

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if let Some(val) = max_proto_depth {
        build.define("JANET_MAX_PROTO_DEPTH", val);
    }

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if let Some(val) = max_macro_expand {
        build.define("JANET_MAX_MACRO_EXPAND", val);
    }

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if let Some(val) = max_stack {
        build.define("JANET_STACK_MAX", val);
    }

    #[cfg(all(
        feature = "link-amalg",
        not(feature = "link-system"),
        feature = "debug-symbols"
    ))]
    build.flag("-ggdb3");

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    build.flag("-fPIC").no_default_flags(true).compile("janet");

    #[cfg(all(feature = "link-amalg", not(feature = "link-system")))]
    if illumos {
        build.flag("-D__EXTENSIONS__");
        build.flag("-DJANET_NO_NANBOX");
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
