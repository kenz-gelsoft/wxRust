use std::libc::*;


pub struct ColorRGB {
    r:u8,
    g:u8,
    b:u8,
}

pub struct Point {
    x:c_int,
    y:c_int,
}
pub struct LongPoint {
    x:c_long,
    y:c_long,
}
pub struct Vector {
    x:c_int,
    y:c_int,
}

pub struct Size {
    w:c_int,
    h:c_int,
}
pub struct DoubleSize {
    w:c_double,
    h:c_double,
}

pub struct Rect {
    x:c_int,
    y:c_int,
    w:c_int,
    h:c_int,
}
