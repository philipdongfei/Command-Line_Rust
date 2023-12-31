impl<T, E: ::std::fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self) {
            Result::Ok(val) => val,
            Result::Err(err) => 
                panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),

        }
    }
}
