#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "turtle_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtle_interfaces__srv__SwitchMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtle_interfaces__rosidl_generator_c")]
extern "C" {
    fn turtle_interfaces__srv__SwitchMode_Request__init(msg: *mut SwitchMode_Request) -> bool;
    fn turtle_interfaces__srv__SwitchMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Request>, size: usize) -> bool;
    fn turtle_interfaces__srv__SwitchMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Request>);
    fn turtle_interfaces__srv__SwitchMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Request>) -> bool;
}

// Corresponds to turtle_interfaces__srv__SwitchMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i32,

}



impl Default for SwitchMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtle_interfaces__srv__SwitchMode_Request__init(&mut msg as *mut _) {
        panic!("Call to turtle_interfaces__srv__SwitchMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtle_interfaces/srv/SwitchMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtle_interfaces__srv__SwitchMode_Request() }
  }
}


#[link(name = "turtle_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtle_interfaces__srv__SwitchMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtle_interfaces__rosidl_generator_c")]
extern "C" {
    fn turtle_interfaces__srv__SwitchMode_Response__init(msg: *mut SwitchMode_Response) -> bool;
    fn turtle_interfaces__srv__SwitchMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Response>, size: usize) -> bool;
    fn turtle_interfaces__srv__SwitchMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Response>);
    fn turtle_interfaces__srv__SwitchMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwitchMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SwitchMode_Response>) -> bool;
}

// Corresponds to turtle_interfaces__srv__SwitchMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwitchMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SwitchMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtle_interfaces__srv__SwitchMode_Response__init(&mut msg as *mut _) {
        panic!("Call to turtle_interfaces__srv__SwitchMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwitchMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtle_interfaces__srv__SwitchMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwitchMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwitchMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtle_interfaces/srv/SwitchMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtle_interfaces__srv__SwitchMode_Response() }
  }
}






#[link(name = "turtle_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtle_interfaces__srv__SwitchMode() -> *const std::ffi::c_void;
}

// Corresponds to turtle_interfaces__srv__SwitchMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SwitchMode;

impl rosidl_runtime_rs::Service for SwitchMode {
    type Request = SwitchMode_Request;
    type Response = SwitchMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtle_interfaces__srv__SwitchMode() }
    }
}


