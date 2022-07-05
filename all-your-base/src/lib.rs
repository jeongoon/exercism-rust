#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let checked_base = |b, error_code| {
        if b <= 1 {
            Err(error_code)
        } else {
            Ok(b)
        }
    };

    checked_base(from_base, Error::InvalidInputBase)
        .and(checked_base(to_base, Error::InvalidOutputBase))
        .and({
            let mut maybe_error: Option<Error> = None;

            number
                .iter()
                .try_fold(0, |n: u32, x| {
                    if x >= &from_base {
                        maybe_error = Some(Error::InvalidDigit(*x));
                        None
                    } else {
                        n.checked_mul(from_base)
                            .and_then(|m| m.checked_add(*x))
                            .or_else(|| {
                                maybe_error = Some(Error::InvalidDigit(*x));
                                None
                            })
                    }
                })
                .ok_or_else(|| maybe_error.unwrap())
        })
        .map(|mut n| {
            let mut converted: Vec<u32> = vec![];
            loop {
                let n_ = n / to_base;
                converted.insert(0, n - n_ * to_base);
                if n_ == 0 {
                    break;
                }
                n = n_;
            }
            converted
        })
}
