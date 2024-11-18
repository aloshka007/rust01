#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    
}


fn main() {
    check_size([0u8; 767]);  // Size is 767 * 1 byte = 767 bytes.
    check_size([0i32; 191]); // Size is 191 * 4 bytes = 764 bytes.
    
    
    check_size(["hello你好"; 2]); // Each `&str` takes 16 bytes, so total is 16 * 2 = 32 bytes.
    
   
    check_size([(); 2].map(|_| "hello你好".to_string())); 
    
    
    check_size(['中'; 2]); // 2 * 4 bytes = 8 bytes.

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
