use std::path::PathBuf;

extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
    .header("bindings.h")
    .header("../../../src/compat/libc/include/string.h")
    .header("../../../src/compat/libc/include/stdio.h")
    .header("../../../src/compat/libc/include/stdlib.h")
    .header("../../../src/compat/posix/include/unistd.h")
    .header("../../../src/compat/posix/include/sys/select.h")
    .header("../../../src/drivers/input/input_dev.h")
    // .header("../../../src/util/dlist/dlist_debug.h")
    .header("../../../src/include/util/dlist.h")
    .clang_arg("-I../../../src/compat/posix/include")
    .clang_arg("-I../../../src/compat/libc/include")
    .clang_arg("-I../../../src/include")
    // .clang_arg("-I../../../src/drivers/input")
    .clang_arg("-I../../../src/arch/x86/include")
    // ВАЖНО! т.к. emlibc-проект исключительно для подготовки биндингов - обязательно используем хост-цель
    .clang_arg("--target=x86_64-pc-linux-gnu")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .ctypes_prefix("::core::ffi")
    .use_core()
    .emit_builtins()
    .layout_tests(false)
    .raw_line("#![no_std]")
    .raw_line("#![feature(alloc_error_handler)]")
    .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused)]")
    .raw_line("pub mod std;")
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("lib.rs"))
        .expect("Couldn't write bindings!");    
}