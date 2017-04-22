extern crate lib1;
extern crate lib2;
extern crate lib3;

fn main() {
    println!("{0}, world!", lib2::unexclamate(lib1::hello()) );
}
