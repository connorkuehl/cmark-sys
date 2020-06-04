#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi;

    #[test]
    fn cmark_simple_interface() {
        let c_str = ffi::CString::new("Thank you, **CommonMark!**").unwrap();
        let actual = unsafe {
            cmark_markdown_to_html(
                c_str.as_ptr(),
                c_str.as_bytes().len() as _,
                CMARK_OPT_DEFAULT as _,
            )
        };
        let actual = unsafe { ffi::CString::from_raw(actual) };

        assert_eq!(
            actual,
            ffi::CString::new("<p>Thank you, <strong>CommonMark!</strong></p>\n").unwrap()
        );
    }
}
