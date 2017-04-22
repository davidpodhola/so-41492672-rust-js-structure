extern crate lib1;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn unexclamate(s:String) -> String {
    s.replace("!", "")
}
