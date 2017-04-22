#![feature(link_args)]
#[link_args = "-s EXPORTED_FUNCTIONS=['_hello']"]
extern {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub fn hello() -> String {
    "Hello!".to_string()
}

fn main() {
    /* Intentionally left blank */
}
