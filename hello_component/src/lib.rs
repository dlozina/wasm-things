#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

// Keep in mind that you need a host code to run this component code
impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
