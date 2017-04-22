#![feature(link_args)]
#[link_args = "-s EXPORTED_FUNCTIONS=['_html1']"]
extern {}

extern crate webplatform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub fn html1(s:String) -> String {
    let document = webplatform::init();    
    let body = document.element_query("body").unwrap();
    let hr = document.element_create("hr").unwrap();
    hr.html_append( &s );
    body.append(&hr);
    body.html_get() 
}    

fn main() {
    /* Intentionally left blank */
}
