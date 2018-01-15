#[macro_use]
extern crate libasm;

lasm! {
    "x86_64-unknown-linux-gnu"
    fn add2 {
        mov %rax, %rdi
        add %rax, %rsi
    }

    "x86_64-pc-windows-msvc"
    fn add2 {
        mov rax, rcx
        add rax, rdx
    }
}

extern "C" {
    fn add2(a: u64, b: u64) -> u64;
}

fn main() {
    let x = unsafe { add2(3, 4) };
    println!("Hello, world! 3 + 4 = {}", x);
}
