
extern crate libc; // use libc::size_t;

mod ecnl_endpoint;

#[allow(unused_parens)]
#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use crate::ecnl_endpoint::ept;

    fn dump_ept(ept: *mut ept::ecnl_endpoint_t) {
        unsafe {
            let ept_port_id = (*ept).ept_port_id;
            let ept_name = CStr::from_ptr((*ept).ept_name).to_string_lossy().into_owned();
            let ept_up_down = (*ept).ept_up_down;
            let xx = if (ept_up_down != 0) { "UP" } else { "DOWN" };

            println!("ept {} {} {}", ept_port_id, ept_name, xx);
            println!();
        }
    }

    #[test]
    fn it_works() {
        unsafe {
            let num_ports = ept::ecnl_init();
            for port_id in 0..num_ports {
                let ept = ept::ept_create(port_id as u32);
                dump_ept(ept);

                ept::ept_update(ept);
                dump_ept(ept);

                // pub fn ept_do_read_async(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t);
                // pub fn ept_do_read(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t, nsecs: ::std::os::raw::c_int);
                // pub fn ept_do_xmit(ept: *mut ecnl_endpoint_t, buf: *mut buf_desc_t);

                ept::ept_destroy(ept);
            }
        }


        assert_eq!(2 + 2, 4);
    }
}
