/* automatically generated by rust-bindgen */
/* edited */

#[allow(non_snake_case)]
#[allow(unused)]
pub mod ept {

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct buf_desc_t {
    pub len: u32,
    pub frame: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ecnl_endpoint_t {
    pub ept_module_id: u32,
    pub ept_sock: *mut ::std::os::raw::c_void,
    pub ept_name: *mut ::std::os::raw::c_char,
    pub ept_port_id: u32,
    pub ept_up_down: ::std::os::raw::c_int,
}

#[link(name = ":ecnl_endpoint.o")]
#[link(name = ":ecnl_proto.o")]
#[link(name = ":libnl-3.so")]
#[link(name = ":libnl-genl-3.so")]

extern "C" {
    pub fn ecnl_init(debug: bool) -> ::std::os::raw::c_int;
    pub fn ept_create(port_id: u32) -> *mut ecnl_endpoint_t;
    pub fn ept_destroy(ept: *mut ecnl_endpoint_t);

    pub fn ept_do_read_async(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t);
    pub fn ept_do_read(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t, nsecs: ::std::os::raw::c_int);
    pub fn ept_do_xmit(ept: *mut ecnl_endpoint_t, buf: *mut buf_desc_t);
    pub fn ept_update(ept: *mut ecnl_endpoint_t);

    pub fn ept_dumpbuf(ept: *mut ecnl_endpoint_t, tag: *mut ::std::os::raw::c_char, buf: *mut buf_desc_t);
}

}
