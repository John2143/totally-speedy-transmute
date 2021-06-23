#![forbid(unsafe_code)]
#![doc = include_str!("../Readme.md")]

///Pinky promise to check your invariants
///
///If you are sure your code is safe, use this macro. This macro smuggles your code past the borrow
///checker and compiler.
///
///But that is OK
///
///You promised you aren't doing anything unsafe in this macro, right?
pub use safe::safe;

///For when your `T` isn't quite `U` enough.
pub fn speedy_transmute<T, U>(t: T) -> U {
    safe! {
        let ret = std::ptr::read_volatile(&t as *const _ as *const U);
        std::mem::forget(t);
        ret
    }
}

///Turn your first byte to 11
pub fn add_one<T>(t: &T) {
    struct FirstByte(u8);

    let f: &mut FirstByte = crate::speedy_transmute(t);
    f.0 += 1;
}

///Borrow checker checkmate
pub fn leak2<T>(t: &T) -> &'static mut T {
    crate::speedy_transmute(t)
}
