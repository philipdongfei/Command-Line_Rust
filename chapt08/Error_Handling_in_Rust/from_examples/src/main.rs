fn main() {
    let string: String = From::from("foo");
    let bytes: Vec<u8> = From::from("foo");
    let cow: ::std::borrow::Cow<str> = From::from("foo");
}
