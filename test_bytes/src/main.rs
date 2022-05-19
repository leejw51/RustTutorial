
fn main() {
    
    let b= bytes::Bytes::from(vec![0x31,0x32,0x33, 0x34, 0x35]);
    let c= bytes::Bytes::from(&b"12345"[..]);
    let d= Vec::from(&b"12345"[..]);
    println!("0x{:x}", b);    
    println!("0x{:x}", c);        
    println!("{:?}", d.as_slice());
    println!("{:?}", &d[..]);
}
