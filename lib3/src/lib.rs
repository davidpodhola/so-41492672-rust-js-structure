extern crate webplatform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn html1(s:String) -> String {
    let document = webplatform::init();    
    let body = document.element_query("body").unwrap();
    let hr = document.element_create("hr").unwrap();
    hr.html_append( &s );
    body.append(&hr);
    body.html_get() 
}    
