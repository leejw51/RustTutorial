
use std::ffi::CString;
pub type MyCallback=extern fn(*const u8) -> i32;
#[repr(C)]
pub struct Fruit {
    pub price:i64,
    pub call_back: extern fn(*const u8) -> i32,
}

impl Fruit {
    pub  fn show(&mut self)
    {
        println!("fruit show {}", self.price);
    }
}

#[no_mangle]
pub unsafe extern "C" fn display( f: *mut Fruit) {    
    (*f).show();
    println!("activate callback start");
   let a= String::from(format!("OK {}", (*f).price));
   // let b= a.as_str();
   let src_string = CString::new(a.as_bytes()).expect("get cstring");
   let src = src_string.to_bytes_with_nul();
    ((*f).call_back)(src.as_ptr());    
    println!("activate callback end");
} 


#[no_mangle]
pub unsafe extern "C" fn set_callback( f: *mut Fruit,call_back: MyCallback) {    
    println!("set activate callback");
    (*f).call_back =call_back;
} 



