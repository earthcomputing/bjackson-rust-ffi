
extern crate libc; // use libc::size_t;

mod ecnl_endpoint;

#[cfg(test)]
mod tests {
    use crate::ecnl_endpoint::ept;

    #[test]
    fn it_works() {
        unsafe {
            let num_ports = ept::ecnl_init();
        }
        assert_eq!(2 + 2, 4);
    }
}

// pub fn ept_create(port_id: u32) -> *mut ecnl_endpoint_t;
// pub fn ept_destroy(ept: *mut ecnl_endpoint_t);

// pub fn ept_do_read_async(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t);
// pub fn ept_do_read(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t, nsecs: ::std::os::raw::c_int);
// pub fn ept_do_xmit(ept: *mut ecnl_endpoint_t, buf: *mut buf_desc_t);
// pub fn ept_update(ept: *mut ecnl_endpoint_t);
