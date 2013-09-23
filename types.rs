use std::libc::*;

//
pub struct Rect {
    x:int,
    y:int,
    w:int,
    h:int,
}

pub struct IntPoint {
    x:int,
    y:int,
}

pub struct IntSize {
    w:int,
    h:int,
}

pub struct LongPoint {
    x:c_long,
    y:c_long,
}

pub struct ColorRGB {
    r:u8,
    g:u8,
    b:u8,
}

pub struct DoubleSize {
    x:c_double,
    y:c_double,
}

pub struct Vector {
    x:int,
    y:int,
}