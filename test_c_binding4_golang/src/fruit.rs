use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
pub type MyCallback = extern "C" fn(*const u8) -> i32;
#[repr(C)]
pub struct Fruit {
    pub price: i64,
    pub call_back: extern "C" fn(*const u8) -> i32,
}

impl Fruit {
    pub fn show(&mut self) {
        println!("fruit show {}", self.price);
    }
}

#[no_mangle]
pub unsafe extern "C" fn display(f: *mut Fruit) {
    (*f).show();
    println!("activate callback start");
    let a = String::from(format!("OK {}", (*f).price));
    // let b= a.as_str();
    let src_string = CString::new(a.as_bytes()).expect("get cstring");
    let src = src_string.to_bytes_with_nul();
    ((*f).call_back)(src.as_ptr());
    println!("activate callback end");
}

#[no_mangle]
pub unsafe extern "C" fn set_callback(f: *mut Fruit, call_back: MyCallback) {
    println!("set activate callback");
    (*f).call_back = call_back;
}

#[no_mangle]
pub unsafe extern "C" fn add(f: i32) -> i32 {
    f + 100
}

pub unsafe fn get_string(src: *const c_char) -> String {
    CStr::from_ptr(src).to_string_lossy().into_owned()
}

#[no_mangle]
pub unsafe extern "C" fn add_text(src: *const c_char, output: *mut u8, output_length: u32) -> i32 {
    let dst = format!("new string={}", get_string(src));
    ptr::write_bytes(output, 0, output_length as usize);
    let new_length = dst.as_bytes().len() as i32;
    ptr::copy_nonoverlapping(dst.as_bytes().as_ptr(), output, new_length as usize);

    new_length
}
