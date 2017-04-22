#![feature(link_args)]
#[link_args = "-s EXPORTED_FUNCTIONS=['_unexclamate']"]
extern {}

extern crate lib1;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub fn unexclamate(s:String) -> String {
    s.replace("!", "")
}

fn main() {
    /* Intentionally left blank */
}
