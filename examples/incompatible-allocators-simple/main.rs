#[repr(C)]
pub struct Buffer {
  ptr : *mut u32,
  len : usize,
}
extern "C" {
  fn free_in_cpp(ptr : *mut u32, len : usize);
}

fn main() {
  //Create a vector with memory allocation in Rust
  let mut vec = vec![1,2,3,4,5];

  let ptr = vec.as_mut_ptr();
  let len = vec.len();

  //To prevent Rust from freeing it
  std::mem::forget(vec);

  unsafe {
    println!("Passing Rust allocated memory to C++");
    free_in_cpp(ptr, len);
  }

  println!("Done but this is undefined behaviour")
}