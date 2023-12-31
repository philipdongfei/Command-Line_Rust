pub fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None, 
        Some(value) => Some(f(value)),
    }
}
