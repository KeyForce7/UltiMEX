#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(llvm_asm)]

mod ultimex;



#[skyline::main(name = "acmd_test")]
pub fn main() {
    ultimex::install();
}