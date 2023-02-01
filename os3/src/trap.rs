use core::arch::global_asm;

use riscv::register::{mtvec::TrapMode, stvec};

mod context;

pub use context::TrapContext;

global_asm!(include_str!("trap/trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}


#[no_mangle]
pub fn trap_handler(cx:&mut TrapContext) -> &mut TrapContext {
    cx
}