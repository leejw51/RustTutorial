use ring::digest::{Context, Digest, SHA256};
use std::io;
fn main() {
    println!("Hello, world!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("length {}", name.len());
    let read_bytes= hex::decode(&name.as_bytes()[..64]).unwrap();
    println!("read bytes {:?}", read_bytes);
    println!("restored {}", hex::encode(&read_bytes));
    let target2=read_bytes.clone();
    assert!(target2.len()==32);

    let s='0' as u8;
    let e= '9' as u8;
    let n=8;
    let mut buffer  = vec![s;n];
    let current =buffer.len()-1;
    let mut done =false;

   while !done {
        let mut context = Context::new(&SHA256);
        //println!("{:?}", buffer);
        context.update(&buffer[..n]);
        let src= context.finish();
        let src2= src.as_ref();
        assert!(src2.len()==32);
        //println!("{:?}", hex::encode(&src2));
        if &src2[..]== &target2[..] {
            let m =std::str::from_utf8(&buffer).unwrap();
            println!("found {}", m);
            println!("found {:?}", hex::encode(&src2));
            done=true;
        }
        else {
           
           // println!("different");
           // println!("{}\n{}", hex::encode(&src2), hex::encode(&target2));
        }

        // increase
        buffer[current] = buffer[current] + 1;
        //for j := current; j >= 0; j-- 
        for i in 0.. buffer.len() 
        {
            let j= (buffer.len() -1) -i;

			if buffer[j] > e {
				buffer[j] = s;
				if j >= 1 {
					buffer[j-1] = buffer[j-1] + 1;
				} else {
					done = true
				}
			}

		}

    }
}

