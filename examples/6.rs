// from std::result::Result
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

// from std::option::Option
pub enum Option<T> {
    None,
    Some(T),
}

// from std::mem::drop
pub fn drop<T>(_x: T) {}
