//!

#![feature(type_name_of_val)]

use std::collections::HashMap;

use miner_macro::mine_methods;

pub struct Foo;

#[mine_methods] // <-- Generates a `foo_methods` module in the parent module
impl Foo {
    pub fn foo() {}

    pub fn bar() {}
}

pub struct Bar;
pub struct StructName;

#[mine_methods] // <-- Generates a `bar_methods` module in the parent module
impl Bar {
    pub fn foo(_: &mut StructName, _: HashMap<String, String>) -> anyhow::Result<()> {
        println!("called from Bar::foo");
        Ok(())
    }

    pub fn bar(_: &mut StructName, _: HashMap<String, String>) -> anyhow::Result<()> {
        println!("called from Bar::bar");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name_of_val;

    #[test]
    fn test_foo() {
        println!("methods in Foo:");
        for method in super::foo_methods::METHODS.iter() {
            println!("{method}");
        }
        for method in super::foo_methods::FN_POINTERS.iter() {
            (method)(); // method call
        }
        for (fn_name, f) in super::foo_methods::FN_MAP.iter() {
            println!("{fn_name} {}", type_name_of_val(f));
        }
    }

    #[test]
    fn test_bar() {
        use super::{HashMap, StructName};
        println!("methods in Bar:");
        for method in super::bar_methods::METHODS.iter() {
            println!("{method}");
        }
        let (arg1, arg2) = (&mut StructName, HashMap::new());
        for method in super::bar_methods::FN_POINTERS.iter() {
            (method)(arg1, arg2.clone()).unwrap(); // method call
        }
        for (fn_name, f) in super::bar_methods::FN_MAP.iter() {
            println!("###{fn_name}### {}", type_name_of_val(f));
        }
    }
}
