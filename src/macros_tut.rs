
macro_rules! custom_macro {
    () => {
        println!("this is an custom declarative macro")
    };
}

pub fn macros() {
    custom_macro!()
}