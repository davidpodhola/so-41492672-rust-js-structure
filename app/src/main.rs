extern crate lib1;
extern crate lib2;
extern crate lib3;

fn main() {    
    println!("{0}, world!",Some(lib1::hello()).map(|x| lib2::unexclamate(x)).unwrap() );
}
