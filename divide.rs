// Function to divide two integers, returning a Result
// to handle division by zero cases
pub fn divide(a: i32, b: i32) -> Result<f32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a as f32 / b as f32)
    }
}
