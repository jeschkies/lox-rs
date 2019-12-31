pub type Value = f64;

// We are not repeating the array implementation.
pub type ValueArray = Vec<Value>;

pub fn print_value(value: Value) {
    // {} will print 100.0 as 100.
    print!("{}", value);
}
