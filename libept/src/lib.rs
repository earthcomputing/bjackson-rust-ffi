
extern crate libc; // use libc::size_t;

mod ecnl_endpoint;

#[allow(unused_parens)]
#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
    use std::ffi::{CString, CStr};
    use crate::ecnl_endpoint::ept;

    fn build_asciz_buf() -> ept::buf_desc_t {
        let str : &'static str = "Plain Text Message"; // 506c61696e2054657874204d65737361676500
        let encoded : std::str::EscapeDefault<'_> = str.escape_default(); // escape_unicode();
        let string : std::string::String = encoded.to_string();
        let c_string : std::ffi::CString = CString::new(string).unwrap();
        let bytes : &[u8] = c_string.as_bytes_with_nul();
        let len : usize = bytes.len(); // includes NUL
        assert_eq!(19, len);
        let asciz_FRAME : *const u8 = bytes.as_ptr();
        std::mem::forget(c_string); // rather than bytes
        let asciz_buf : ept::buf_desc_t = ept::buf_desc_t{len: len as u32, frame: asciz_FRAME as *mut _}; // *mut u8
        // println!("len {}", asciz_buf.len);
        return asciz_buf;
    }

    // extra test - full binary buffer
    // char ecad_data[EC_MESSAGE_MAX]; // 9000
    fn build_blob_buf() -> ept::buf_desc_t {
        const len : usize = 9000 / 2;
        let mut ary: Vec<u16> = vec![0; len];
        for i in 0..len { ary[i] = i as u16; } // might want: i | 0x8080 ?
        let ary_FRAME : *mut u16 = ary.as_mut_ptr();
        const shortened : usize = 1500 + 26; // MTU + ethernet header
        unsafe {
            let blob_FRAME = std::mem::transmute::<*mut u16, *mut u8>(ary_FRAME); // magic 'cast'
            let blob_buf : ept::buf_desc_t = ept::buf_desc_t{len: shortened as u32, frame: blob_FRAME};
            std::mem::forget(ary);
            return blob_buf;
        }
    }

    // CStr::from_bytes_with_nul

    fn dump_ept(ept: *mut ept::ecnl_endpoint_t) {
        unsafe {
            let ept_port_id = (*ept).ept_port_id;
            let ept_name = CStr::from_ptr((*ept).ept_name).to_string_lossy().into_owned();
            let ept_up_down = (*ept).ept_up_down;
            let carrier = if (ept_up_down != 0) { "UP" } else { "DOWN" };

            println!("ept {} {} {}", ept_port_id, ept_name, carrier);
            println!();
        }
    }

    fn dump_buf(tag: &'static str, buf: ept::buf_desc_t) {
        let len : u32 = buf.len;
        let frame : *const u8 = buf.frame;
        println!("{} buf {}", tag, len);
/*
        unsafe {
            let p : [u8; 9000] = std::mem::transmute::<*const u8, [u8; 9000]>(frame);
            // for i in 0..len as usize { }
            // println!("frame {}", *frame);
        }
*/
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

                let asciz_buf = build_asciz_buf();
                let blob_buf = build_blob_buf();

                dump_buf("asciz", asciz_buf);
                dump_buf("blob", blob_buf);
                println!();

                let p1 : *const ept::buf_desc_t = &asciz_buf;
                let p2 : *const ept::buf_desc_t = &blob_buf;

                ept::ept_do_xmit(ept, p1 as *mut _);
                ept::ept_do_xmit(ept, p2 as *mut _);

                // pub fn ept_do_read_async(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t);
                // pub fn ept_do_read(ept: *mut ecnl_endpoint_t, actual_buf: *mut buf_desc_t, nsecs: ::std::os::raw::c_int);

                let num_secs = 10;
                ept::ept_do_read(ept, p2 as *mut _, num_secs);

                ept::ept_destroy(ept);
            }
        }


        assert_eq!(2 + 2, 4);
    }
}
