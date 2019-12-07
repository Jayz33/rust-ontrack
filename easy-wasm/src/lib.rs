extern {
    fn helloWorld();
    fn alert(s: &str);
}

#[no_mangle]
pub extern "C" fn get_number() -> i32 {
    123
}

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn call_hello_world() {
    helloWorld();
}

#[no_mangle]
pub unsafe extern "C" fn greeting() {
    alert("Hello from Rust!");
}

