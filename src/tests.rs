use super::*;

#[test]
fn test_cudnn_create() {
  let mut cudnn_context = unsafe { std::ptr::null_mut() };
  let status = unsafe { cudnnCreate(&mut cudnn_context as *mut *mut cudnnContext) };
  assert_eq!(status, cudnnStatus_t::CUDNN_STATUS_SUCCESS);
}
