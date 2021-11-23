#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(llvm_asm)]
#[allow(unused_variables)]
#[allow(unused_imports)]

mod ultimex;



#[skyline::main(name = "acmd_test")]
pub fn main() {
    ultimex::install();
}