match ::std::ops::Try::into_result(x) {
    Ok(v) => v,
    Err(e) => return ::std::ops::Try::from_error(From::from(e)),
}

