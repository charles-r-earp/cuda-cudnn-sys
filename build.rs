#![allow(warnings)]
use static_assertions::assert_cfg;
use std::{
  env,
  ffi::CString,
  path::{Path, PathBuf},
  process::Command,
};

#[cfg(target_os = "linux")]
fn linux_locate_parent_dir(name: impl AsRef<str>) -> String {
  let path = Command::new("locate")
    .arg(name.as_ref())
    .arg("--limit")
    .arg("1")
    .output()
    .unwrap()
    .stdout;
  let path = CString::new(path).unwrap().into_string().unwrap();
  let path = path.lines().next().unwrap();
  Path::new(path)
    .parent()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string()
}

fn main() {
  assert_cfg!(target_os = "linux", "Only Linux supported!");

  let cuda_include = {
    // locate and link cuda libs and headers
    if cfg!(target_os = "linux") {
        let cudnn_lib = linux_locate_parent_dir("libcudnn.so");
        println!("cargo:rustc-link-search=native={}", cudnn_lib);
        println!("cargo:rustc-link-lib=dylib=cudnn");
        linux_locate_parent_dir("include/cuda.h")
    } else {
        unreachable!()
    }
  };

  let bindings = bindgen::Builder::default()
    .raw_line("#![allow(warnings)]")
    .raw_line("mod tests;")
    .header("wrapper.h")
    .clang_arg("-I")
    .clang_arg(cuda_include)
    .ctypes_prefix("::libc")
    .size_t_is_usize(true)
    .rustified_non_exhaustive_enum("cudnn[A-Za-z]+_t")
    .rustified_non_exhaustive_enum("cuda.*")
    .whitelist_function("cu.*")
    .whitelist_var("CUDNN.*")
    .whitelist_type("[Cc][Uu].*")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .rustfmt_bindings(true)
    .generate()
    .unwrap();
  bindings.write_to_file("src/lib.rs").unwrap();
}
