#[macro_export]
macro_rules! impl_perf_instr {
    (@variant $name:ident, $instr:expr) => {
        paste::paste! {
            
        }
    };

    ($name:ident, ($j:pat, $i:ident) = (_, $unroll:literal), $instr64:expr, $instr32:expr) => {
        paste::paste! {
            pub fn [<perf_ $name>]() {
                use crate::LOOP_NUM;
                const TOTAL_NUM: usize = LOOP_NUM * $unroll;

                let total64 = unsafe { [<perf_ $name 64>]() };
                let total32 = unsafe { [<perf_ $name 32>]() };
                // let mul64_total = unsafe { mul64() };
            
                println!("{}: {}, {} cycle/instruction", stringify!([<$name 64>]), total64, total64 as f64 / TOTAL_NUM as f64);
                println!("{}: {}, {} cycle/instruction", stringify!([<$name 32>]), total32, total32 as f64 / TOTAL_NUM as f64);
                println!("{}/{}: {}\n", stringify!([<$name 64>]), stringify!([<$name 32>]), total64 as f64 / total32 as f64);
            }

            #[no_mangle]
            #[inline(never)]
            pub unsafe fn [<perf_ $name 32>]() -> u64 {
                
                let start = std::arch::x86_64::_rdtsc();
            
                for $j in 0..crate::LOOP_NUM {
                    ::seq_macro::seq!($i in 0..=$unroll {
                        $instr32;
                    });
                }

                let end = std::arch::x86_64::_rdtsc();
                return end - start;    
            }

            #[no_mangle]
            #[inline(never)]
            pub unsafe fn [<perf_ $name 64>]() -> u64 {
                let start = std::arch::x86_64::_rdtsc();
            
                for $j in 0..crate::LOOP_NUM {
                    ::seq_macro::seq!($i in 0..=$unroll {
                        $instr64;
                    });
                }

                let end = std::arch::x86_64::_rdtsc();
                return end - start;    
            }
        }
    }
}
