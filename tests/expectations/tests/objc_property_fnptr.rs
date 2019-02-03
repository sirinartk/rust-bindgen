/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo {
    unsafe fn func(
        self,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_short,
            arg3: f32,
        ) -> ::std::os::raw::c_int,
    >;
    unsafe fn setFunc_(
        self,
        func: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    );
}
impl Foo for id {
    unsafe fn func(
        self,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_short,
            arg3: f32,
        ) -> ::std::os::raw::c_int,
    > {
        msg_send!(self, func)
    }
    unsafe fn setFunc_(
        self,
        func: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    ) {
        msg_send!(self, setFunc: func)
    }
}
