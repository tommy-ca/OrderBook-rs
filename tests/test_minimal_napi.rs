// Minimal NAPI test to isolate Node.js v22 compatibility
use napi_derive::napi;

#[napi]
fn hello() -> String {
    "Hello from NAPI!".to_string()
}

#[napi]
pub struct SimpleStruct {
    value: i32,
}

#[napi]
impl SimpleStruct {
    #[napi(constructor)]
    pub fn new(value: i32) -> Self {
        SimpleStruct { value }
    }

    #[napi]
    pub fn get_value(&self) -> i32 {
        self.value
    }
}