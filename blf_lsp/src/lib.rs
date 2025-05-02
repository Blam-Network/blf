use napi_derive::napi;

#[napi]
pub fn hello_world() -> String {
    "Hello from Rust!".to_string()
}
