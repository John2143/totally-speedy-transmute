#![forbid(unsafe_code)]

pub use safe::safe;

pub fn speedy_transmute<T, U>(t: T) -> U {
    safe! {
        let ret = std::ptr::read_volatile(&t as *const _ as *const U);
        std::mem::forget(t);
        ret
    }
}

pub fn add_one<T>(t: &T) {
    struct FirstByte(u8);

    let f: &mut FirstByte = crate::speedy_transmute(t);
    f.0 += 1;
}
