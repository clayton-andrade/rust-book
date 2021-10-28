use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f32) -> f32;
}

static mut ID: u32 = 1;
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("num = {}, r1 = {}, r2 = {}", num, unsafe { *r1 }, unsafe { *r2 });

    num = 10;
    println!("num = {}, r1 = {}, r2 = {}", num, unsafe { *r1 }, unsafe { *r2 });

    unsafe {
        *r2 = 20;
        println!("num = {}, r1 = {}, r2 = {}", num, *r1, *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(b, &[4, 5, 6]);

    let (a, b) = split_at_mut(r, 2);
    println!("a = {:?}, b = {:?}", a, b);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("Square root of 9: {}", sqrt(9.0));
    }

    
    unsafe { 
        println!("ID = {}", ID);
        ID = 2;
        println!("ID = {}", ID);
    }

    add_to_counter(5);
    unsafe {
        println!("COUNTER = {}", COUNTER);
    }
}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}