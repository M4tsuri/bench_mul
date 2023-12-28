#![feature(asm_const)]

use core::arch::asm;
use core_affinity::CoreId;

use clap::Parser;

#[macro_use]
mod macros;

pub const LOOP_NUM: usize = 640000;
pub const UNROLL_NUM: usize = 5_000;
pub const TOTAL_NUM: usize = LOOP_NUM * UNROLL_NUM;

pub const MEM_AREA_OFFSET: u64 = 0x1000000;
/// 128 pages, 512 KB
pub const MEM_AREA_LEN: usize = 256 * 0x1000;

#[derive(Debug, Parser)]
pub enum InstrType {
    MUL,
    IMUL,
    MULCACHE,
    IMULCACHE,
    CACHE
}


fn main() {
    let args = InstrType::parse();
    core_affinity::set_for_current(CoreId { id: 0 });
    

    match args {
        InstrType::MUL => perf_mul(),
        InstrType::IMUL => perf_imul(),
        InstrType::MULCACHE => {
            let _alloc = region::alloc_at(
                MEM_AREA_OFFSET as *const (), 
                MEM_AREA_LEN, 
                region::Protection::all()
            ).unwrap();
        
            perf_mul_cache();
            drop(_alloc);
        },
        InstrType::IMULCACHE => {
            let _alloc = region::alloc_at(
                MEM_AREA_OFFSET as *const (), 
                MEM_AREA_LEN, 
                region::Protection::all()
            ).unwrap();
        
            perf_imul_cache();
            drop(_alloc);
        },
        InstrType::CACHE => {
            let _alloc = region::alloc_at(
                MEM_AREA_OFFSET as *const (), 
                MEM_AREA_LEN, 
                region::Protection::all()
            ).unwrap();
        
            perf_cache();
            drop(_alloc);
        }
    }
}


impl_perf_instr!(cache, (i, j) = (_, 1024),
    asm!(
        "mul qword ptr [{idx}]", // 64 bit mul
        // idx = in(reg) MEM_AREA_OFFSET as usize + ((i % 512) * 1024 + j % 512),
        idx = in(reg) MEM_AREA_OFFSET as usize + (i % 64 + (j % 64) * 1024),
        out("rax") _,
        out("rdx") _,
        options(nostack)
    ),
    asm!(
        "mul qword ptr [{idx}]", // 64 bit mul
        idx = in(reg) MEM_AREA_OFFSET as usize + ((i % 64) * 1024 + j % 64),
        out("rax") _,
        out("rdx") _,
        options(nostack)
    )
);

impl_perf_instr!(mul_cache, (i, j) = (_, 1024),
    asm!(
        "mul qword ptr [{idx}]", // 64 bit mul
        "mul qword ptr [{idx} + 0x8]", // 64 bit mul
        "mul qword ptr [{idx} + 0x10]", // 64 bit mul
        "mul qword ptr [{idx} + 0x18]", // 64 bit mul
        "mul qword ptr [{idx} + 0x20]", // 64 bit mul
        "mul qword ptr [{idx} + 0x28]", // 64 bit mul
        idx = in(reg) MEM_AREA_OFFSET as usize + (i % 20000 + j) * 48,
        out("rax") _,
        out("rdx") _,
        options(nostack)
    ),
    asm!(
        "mul qword ptr [{idx}]", // 64 bit mul
        "mul qword ptr [{idx} + 0x8]", // 64 bit mul
        "mul qword ptr [{idx} + 0x10]", // 64 bit mul
        "mul qword ptr [{idx} + 0x18]", // 64 bit mul
        "mul qword ptr [{idx} + 0x20]", // 64 bit mul
        "mul dword ptr [{idx} + 0x28]", // 32 bit mul
        idx = in(reg) MEM_AREA_OFFSET as usize + (i % 20000 + j) * 44,
        out("rax") _,
        out("rdx") _,
        options(nostack)
    )
);

impl_perf_instr!(imul_cache, (i, j) = (_, 1024),
    asm!(
        "imul rdx, qword ptr [{idx}]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x8]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x10]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x18]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x20]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x28]", // 64 bit mul
        idx = in(reg) MEM_AREA_OFFSET as usize + (i % 20000 + j) * 48,
        out("rax") _,
        out("rdx") _,
        options(nostack)
    ),
    asm!(
        "imul rdx, qword ptr [{idx}]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x8]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x10]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x18]", // 64 bit mul
        "imul rdx, qword ptr [{idx} + 0x20]", // 64 bit mul
        "imul edx, dword ptr [{idx} + 0x28]", // 32 bit mul
        idx = in(reg) MEM_AREA_OFFSET as usize + (i % 20000 + j) * 44,
        out("rax") _,
        out("rdx") _,
        options(nostack)
    )
);

impl_perf_instr!(mul, (_, _i) = (_, 5_000),
    asm!(
        "mul rcx",
        out("rax") _,
        out("rdx") _,
        options(nomem, nostack)
    ),
    asm!(
        "mul ecx",
        out("rax") _,
        out("rdx") _,
        options(nomem, nostack)
    )
);

impl_perf_instr!(imul, (_, _i) = (_, 5_000),
    asm!(
        "imul rdx, rcx",
        out("rax") _,
        out("rdx") _,
        options(nomem, nostack)
    ),
    asm!(
        "imul edx, ecx",
        out("rax") _,
        out("rdx") _,
        options(nomem, nostack)
    )
);

