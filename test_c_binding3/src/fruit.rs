
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
    f.as_mut().unwrap().show();    
    println!("activate callback");
    ((*f).call_back)("ABC".as_ptr());    
} 


