use std::env;

fn main() {  
  let cuda_include = env::var("CUDA_INCLUDE_DIR")
    .unwrap_or({
    if cfg!(target_family = "unix") {
      String::from("/usr/local/cuda/include")
    } 
    else if cfg!(target_family = "windows") {
      unimplemented!("Set the environmental variable CUDA_INCLUDE_DIR to the path that includes cuda.h")
    }
    else {
      unimplemented!()
    }
  });
  
  println!("cargo:rustc-link-lib=dylib=cudnn");

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
    .expect("Unable to create bindings. You may need to set the appropriate environmental variables: CUDA_INCLUDE_DIR");
  bindings.write_to_file("src/lib.rs").unwrap();
}
