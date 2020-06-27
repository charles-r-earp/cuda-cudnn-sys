[![License-MIT/Apache-2.0](https://img.shields.io/badge/license-MIT/Apache_2.0-blue.svg)](https://github.com/charles-r-earp/autograph/blob/master/LICENSE-APACHE)

# cuda-cudnn-sys
Rust wrapper for cuDNN, created for autograph https://github.com/charles-r-earp/autograph

# Requirements
CUDA https://developer.nvidia.com/cuda-downloads and cuDNN https://developer.nvidia.com/cudnn must be installed. See https://github.com/bheisler/RustaCUDA for additional help. You may need to set the environmental variable CUDA_INCLUDE_DIR if cuda.h is not in "/usr/local/cuda/include" for unix platforms. This variable must be set for Windows.

# Supported Platforms
Tested on Ubuntu-18.04. Should work on Linux, macOS, and Windows. If you have build errors, please create an issue. 

# Tests
Verify that the crate works by running:
```
cargo test
```
