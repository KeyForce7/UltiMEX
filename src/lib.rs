#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod ultimex;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    ultimex::install();
}