use totally_speedy_transmute::{add_one, leak2, safe, speedy_transmute};

#[test]
fn oops_all_mutable() {
    //clearly not mutable x
    let x = 7;

    //give add_one immutable access
    add_one(&x);

    //oops now it's 8, hope they don't notice
    assert_eq!(x, 8);
}

#[test]
fn transmute_u8() {
    //Simple array of some u8s
    let i = &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    //actually, I meant to say i16
    let u: &[i16] = speedy_transmute(i);

    assert_eq!(u[0], 0x0201);
    assert_eq!(u[1], 0x0403);
    assert_eq!(u[2], 0x0605);
    assert_eq!(u[3], 0x0807);
    assert_eq!(u[4], 0x0009);

    //Oh no, length is unitialized
    assert_ne!(u.len(), 10);
    assert_ne!(u.len(), 5);
}

#[test]
fn fast_vec() {
    //nice little box
    let b = Box::new([4u8; 10]);

    assert_eq!(b[6], 4);

    //I just wish it was a little bit bigger. and mutable.
    //But allocating a new box is slow, so just re-use the old one
    let mut bigger_box: Box<[u8; 11]> = speedy_transmute(b);

    //old data is ok
    assert_eq!(bigger_box[6], 4);

    //use that extra byte responsibly
    bigger_box[10] = 100;

    assert_eq!(bigger_box[10], 100);

    //allocator doesn't know PepeLaugh
    drop(bigger_box);
}

#[test]
fn safe() {
    //The heap was OK, but how about in .text?
    let x = &[1, 2, 3] as &[i32];

    //yep, seems good to me
    //SAFETY: your program is probably longer than 400 bytes.
    let _ = safe! { x.get_unchecked(400) };
}

#[test]
fn leek() {
    fn static_enojoyer(s: &mut i32) {
        *s += 2;
    }

    let whereami = {
        let five = 5;

        leak2(&five)
    };

    //whose five is it anyway
    static_enojoyer(whereami);
}
