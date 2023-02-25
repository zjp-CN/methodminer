//!

use miner_macro::mine_methods;


struct Foo;

#[mine_methods] // <-- Generates a `foo_methods` module in the parent module
impl Foo {
    fn foo() -> u8 { 42 }

    fn bar() {}
}



#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {

        println!("methods:");
        for method in super::foo_methods::METHODS.iter() {
            println!("  {method}");
        }
        todo!()
    }
}
