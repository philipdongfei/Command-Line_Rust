fn main() {
    //option_ex_string_find::main_find();
    assert_eq!(option_ex_string_find::extension("foobar.csv").unwrap_or("rs"), "csv"); 
    assert_eq!(option_ex_string_find::extension("foobar").unwrap_or("rs"), "rs"); 
}
