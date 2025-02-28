use pyo3::prelude::*;

/// Validate al number using modulus 10.
#[pyfunction]
fn validate_modulus_10(number: String)->PyResult<bool> {

    // controls whether a digit have to be doubled
    let mut index:i32=-1;
    
    // accrues the sum of all digits of the `number`
    let mut total:i32=0;

    // indicates the 'base' to which a number will be converted
    const RADIX:u32 = 10;

    for character in number.chars().rev() {
        match character.to_digit(RADIX) {
            Some(digit) => {
                // double digit every second digit
                if index % 2 == 0{
                
                    // if doubling a digit yields two digits, subtract 9
                    if digit*2 > 9 {
                        total += digit as i32 * 2 - 9;
                    }
                    else {
                        total += digit as i32 * 2;
                    }
                }
                else {
                    total += digit as i32;
                }
            },
            // character is not a valid digit
            None => return Ok(false),
        };
        index += 1;
    }

    // if the result of modulus is 0 then the number is valid
    if total % 10 == 0{
        return Ok(true)
    }

    Ok(false)
}

/// A Python module implemented in Rust.
#[pymodule]
fn glue_rust_to_python(m: &Bound<'_,PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_modulus_10,m)?)?;
    Ok(())
}