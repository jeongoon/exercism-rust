use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black       = 0,
    Brown       = 1,
    Red         = 2,
    Orange      = 3,
    Yellow      = 4,
    Green       = 5,
    Blue        = 6,
    Violet      = 7,
    Grey        = 8,
    White       = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    unimplemented!("convert a color into a numerical representation")
    //_color::int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok( ret1 ) => ret1,
        Err( ret2 ) => ret2,
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().cloned().collect()
}

fn main() {
    println!("{:?}", color_to_value( ResistorColor::Red ) );
    println!("{:?}", value_to_color_string( 9 ) );
}
