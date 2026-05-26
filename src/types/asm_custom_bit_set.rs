use super::{SetInt, SetIntConstruct};

#[repr(C)]
pub struct AsmCustomBitSet {
    bits: [u64; 1024],
}

impl SetIntConstruct for AsmCustomBitSet {
    fn new() -> Self {
        Self { bits: [0; 1024] }
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
}

impl SetInt for AsmCustomBitSet {
    fn clear(&mut self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut index = 0;
            asm!(
                "pxor xmm0, xmm0",
                "2:",
                "movdqu [ {ptr} + {index} ], xmm0",
                "movdqu [ {ptr} + {index} + 16 ], xmm0",
                "movdqu [ {ptr} + {index} + 32 ], xmm0",
                "movdqu [ {ptr} + {index} + 48 ], xmm0",
                "movdqu [ {ptr} + {index} + 64 ], xmm0",
                "movdqu [ {ptr} + {index} + 80 ], xmm0",
                "movdqu [ {ptr} + {index} + 96 ], xmm0",
                "movdqu [ {ptr} + {index} + 112 ], xmm0",
                "add {index}, 128",
                "cmp {index}, 8192",
                "jb 2b",
                ptr = in(reg) self.bits.as_mut_ptr(),
                index = inout(reg) index => _,
                out("xmm0") _,
                options(nostack, preserves_flags)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let ptr = self.bits.as_mut_ptr();
            asm!(
                "dup v0.4s, wzr",
                "2:",
                "stp q0, q0, [{ptr}], #32",
                "stp q0, q0, [{ptr}], #32",
                "stp q0, q0, [{ptr}], #32",
                "stp q0, q0, [{ptr}], #32",
                "subs {count:w}, {count:w}, #1",
                "b.ne 2b",
                ptr = inout(reg) ptr => _,
                count = inout(reg) 64 => _,
                out("v0") _,
                options(nostack)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            self.bits.fill(0);
        }
    }

    fn insert(&mut self, n: u16) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            asm!(
                "bts qword ptr [{ptr}], {bit}",
                ptr = in(reg) self.bits.as_mut_ptr(),
                bit = in(reg) n as u64,
                options(nostack)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            asm!(
                "lsr {idx}, {bit}, #6",
                "and {offset}, {bit}, #63",
                "mov {mask}, #1",
                "lsl {mask}, {mask}, {offset}",
                "ldr {val}, [{ptr}, {idx}, lsl #3]",
                "orr {val}, {val}, {mask}",
                "str {val}, [{ptr}, {idx}, lsl #3]",
                ptr = in(reg) self.bits.as_mut_ptr(),
                bit = in(reg) n as usize,
                idx = out(reg) _,
                offset = out(reg) _,
                mask = out(reg) _,
                val = out(reg) _,
                options(nostack, preserves_flags)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            self.bits[(n as usize) >> 6] |= 1 << (n & 63);
        }
    }

    fn remove(&mut self, n: u16) -> bool {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let removed: u8;
            asm!(
                "btr qword ptr [{ptr}], {bit}",
                "setc {removed}",
                ptr = in(reg) self.bits.as_mut_ptr(),
                bit = in(reg) n as u64,
                removed = out(reg_byte) removed,
                options(nostack)
            );
            removed != 0
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let removed: u64;
            asm!(
                "lsr {idx}, {bit}, #6",
                "and {offset}, {bit}, #63",
                "mov {mask}, #1",
                "lsl {mask}, {mask}, {offset}",
                "ldr {val}, [{ptr}, {idx}, lsl #3]",
                "and {removed}, {val}, {mask}",
                "bic {val}, {val}, {mask}",
                "str {val}, [{ptr}, {idx}, lsl #3]",
                ptr = in(reg) self.bits.as_mut_ptr(),
                bit = in(reg) n as usize,
                removed = out(reg) removed,
                idx = out(reg) _,
                offset = out(reg) _,
                mask = out(reg) _,
                val = out(reg) _,
                options(nostack, preserves_flags)
            );
            removed != 0
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            let idx = (n as usize) >> 6;
            let mask = 1 << (n & 63);
            let old = self.bits[idx];
            self.bits[idx] &= !mask;
            (old & mask) != 0
        }
    }

    fn contains(&self, n: u16) -> bool {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let present: u8;
            asm!(
                "bt qword ptr [{ptr}], {bit}",
                "setc {present}",
                ptr = in(reg) self.bits.as_ptr(),
                bit = in(reg) n as u64,
                present = out(reg_byte) present,
                options(nostack, readonly)
            );
            present != 0
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let present: u64;
            asm!(
                "lsr {idx}, {bit}, #6",
                "and {offset}, {bit}, #63",
                "mov {mask}, #1",
                "lsl {mask}, {mask}, {offset}",
                "ldr {val}, [{ptr}, {idx}, lsl #3]",
                "and {present}, {val}, {mask}",
                ptr = in(reg) self.bits.as_ptr(),
                bit = in(reg) n as usize,
                present = out(reg) present,
                idx = out(reg) _,
                offset = out(reg) _,
                mask = out(reg) _,
                val = out(reg) _,
                options(nostack, readonly, preserves_flags)
            );
            present != 0
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            (self.bits[(n as usize) >> 6] & (1 << (n & 63))) != 0
        }
    }

    fn len(&self) -> usize {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut count: u64 = 0;
            let mut index = 0;
            asm!(
                "2:",
                "popcnt {tmp}, qword ptr [{ptr} + {index}*8]",
                "add {count}, {tmp}",
                "inc {index}",
                "cmp {index}, 1024",
                "jl 2b",
                ptr = in(reg) self.bits.as_ptr(),
                index = inout(reg) index => _,
                count = inout(reg) count,
                tmp = out(reg) _,
                options(nostack)
            );
            count as usize
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let count: u32;
            let ptr = self.bits.as_ptr();
            asm!(
                "dup v0.8h, wzr",
                "mov {count_reg}, #256",
                "2:",
                "ldp q2, q3, [{ptr}], #32",
                "cnt v2.16b, v2.16b",
                "cnt v3.16b, v3.16b",
                "uaddlp v2.8h, v2.16b",
                "uaddlp v3.8h, v3.16b",
                "add v0.8h, v0.8h, v2.8h",
                "add v0.8h, v0.8h, v3.8h",
                "subs {count_reg}, {count_reg}, #1",
                "b.ne 2b",
                "uaddlv s0, v0.8h",
                "fmov {count:w}, s0",
                ptr = inout(reg) ptr => _,
                count_reg = out(reg) _,
                count = out(reg) count,
                out("v0") _, out("v2") _, out("v3") _,
                options(nostack)
            );
            count as usize
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            self.bits.iter().map(|&w| w.count_ones() as usize).sum()
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = u16> + '_> {
        let mut elems = Vec::new();
        for (i, &word) in self.bits.iter().enumerate() {
            let mut w = word;
            while w != 0 {
                let tz = w.trailing_zeros();
                elems.push((i * 64 + tz as usize) as u16);
                w &= w - 1; // clear lowest set bit
            }
        }
        Box::new(elems.into_iter())
    }

    fn union_with(&mut self, other: &Self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut index = 0;
            asm!(
                "2:",
                "movdqu xmm0, [{self_ptr} + {index}]",
                "movdqu xmm1, [{self_ptr} + {index} + 16]",
                "movdqu xmm2, [{self_ptr} + {index} + 32]",
                "movdqu xmm3, [{self_ptr} + {index} + 48]",
                "movdqu xmm4, [{self_ptr} + {index} + 64]",
                "movdqu xmm5, [{self_ptr} + {index} + 80]",
                "movdqu xmm6, [{self_ptr} + {index} + 96]",
                "movdqu xmm7, [{self_ptr} + {index} + 112]",
                "movdqu xmm8, [{other_ptr} + {index}]",
                "movdqu xmm9, [{other_ptr} + {index} + 16]",
                "movdqu xmm10, [{other_ptr} + {index} + 32]",
                "movdqu xmm11, [{other_ptr} + {index} + 48]",
                "movdqu xmm12, [{other_ptr} + {index} + 64]",
                "movdqu xmm13, [{other_ptr} + {index} + 80]",
                "movdqu xmm14, [{other_ptr} + {index} + 96]",
                "movdqu xmm15, [{other_ptr} + {index} + 112]",
                "por xmm0, xmm8",
                "por xmm1, xmm9",
                "por xmm2, xmm10",
                "por xmm3, xmm11",
                "por xmm4, xmm12",
                "por xmm5, xmm13",
                "por xmm6, xmm14",
                "por xmm7, xmm15",
                "movdqu [{self_ptr} + {index}], xmm0",
                "movdqu [{self_ptr} + {index} + 16], xmm1",
                "movdqu [{self_ptr} + {index} + 32], xmm2",
                "movdqu [{self_ptr} + {index} + 48], xmm3",
                "movdqu [{self_ptr} + {index} + 64], xmm4",
                "movdqu [{self_ptr} + {index} + 80], xmm5",
                "movdqu [{self_ptr} + {index} + 96], xmm6",
                "movdqu [{self_ptr} + {index} + 112], xmm7",
                "add {index}, 128",
                "cmp {index}, 8192",
                "jb 2b",
                self_ptr = in(reg) self.bits.as_mut_ptr(),
                other_ptr = in(reg) other.bits.as_ptr(),
                index = inout(reg) index => _,
                out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
                out("xmm4") _, out("xmm5") _, out("xmm6") _, out("xmm7") _,
                out("xmm8") _, out("xmm9") _, out("xmm10") _, out("xmm11") _,
                out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
                options(nostack)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let self_ptr = self.bits.as_mut_ptr();
            let other_ptr = other.bits.as_ptr();
            asm!(
                "2:",
                "ldp q0, q1, [{self_ptr}]",
                "ldp q2, q3, [{self_ptr}, #32]",
                "ldp q4, q5, [{self_ptr}, #64]",
                "ldp q6, q7, [{self_ptr}, #96]",
                "ldp q8, q9, [{other_ptr}], #32",
                "ldp q10, q11, [{other_ptr}], #32",
                "ldp q12, q13, [{other_ptr}], #32",
                "ldp q14, q15, [{other_ptr}], #32",
                "orr v0.16b, v0.16b, v8.16b",
                "orr v1.16b, v1.16b, v9.16b",
                "orr v2.16b, v2.16b, v10.16b",
                "orr v3.16b, v3.16b, v11.16b",
                "orr v4.16b, v4.16b, v12.16b",
                "orr v5.16b, v5.16b, v13.16b",
                "orr v6.16b, v6.16b, v14.16b",
                "orr v7.16b, v7.16b, v15.16b",
                "stp q0, q1, [{self_ptr}], #32",
                "stp q2, q3, [{self_ptr}], #32",
                "stp q4, q5, [{self_ptr}], #32",
                "stp q6, q7, [{self_ptr}], #32",
                "subs {count:w}, {count:w}, #1",
                "b.ne 2b",
                self_ptr = inout(reg) self_ptr => _,
                other_ptr = inout(reg) other_ptr => _,
                count = inout(reg) 64 => _,
                out("q0") _, out("q1") _, out("q2") _, out("q3") _,
                out("q4") _, out("q5") _, out("q6") _, out("q7") _,
                out("q8") _, out("q9") _, out("q10") _, out("q11") _,
                out("q12") _, out("q13") _, out("q14") _, out("q15") _,
                options(nostack)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            for i in 0..1024 {
                self.bits[i] |= other.bits[i];
            }
        }
    }

    fn intersection_with(&mut self, other: &Self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut index = 0;
            asm!(
                "2:",
                "movdqu xmm0, [{self_ptr} + {index}]",
                "movdqu xmm1, [{self_ptr} + {index} + 16]",
                "movdqu xmm2, [{self_ptr} + {index} + 32]",
                "movdqu xmm3, [{self_ptr} + {index} + 48]",
                "movdqu xmm4, [{self_ptr} + {index} + 64]",
                "movdqu xmm5, [{self_ptr} + {index} + 80]",
                "movdqu xmm6, [{self_ptr} + {index} + 96]",
                "movdqu xmm7, [{self_ptr} + {index} + 112]",
                "movdqu xmm8, [{other_ptr} + {index}]",
                "movdqu xmm9, [{other_ptr} + {index} + 16]",
                "movdqu xmm10, [{other_ptr} + {index} + 32]",
                "movdqu xmm11, [{other_ptr} + {index} + 48]",
                "movdqu xmm12, [{other_ptr} + {index} + 64]",
                "movdqu xmm13, [{other_ptr} + {index} + 80]",
                "movdqu xmm14, [{other_ptr} + {index} + 96]",
                "movdqu xmm15, [{other_ptr} + {index} + 112]",
                "pand xmm0, xmm8",
                "pand xmm1, xmm9",
                "pand xmm2, xmm10",
                "pand xmm3, xmm11",
                "pand xmm4, xmm12",
                "pand xmm5, xmm13",
                "pand xmm6, xmm14",
                "pand xmm7, xmm15",
                "movdqu [{self_ptr} + {index}], xmm0",
                "movdqu [{self_ptr} + {index} + 16], xmm1",
                "movdqu [{self_ptr} + {index} + 32], xmm2",
                "movdqu [{self_ptr} + {index} + 48], xmm3",
                "movdqu [{self_ptr} + {index} + 64], xmm4",
                "movdqu [{self_ptr} + {index} + 80], xmm5",
                "movdqu [{self_ptr} + {index} + 96], xmm6",
                "movdqu [{self_ptr} + {index} + 112], xmm7",
                "add {index}, 128",
                "cmp {index}, 8192",
                "jb 2b",
                self_ptr = in(reg) self.bits.as_mut_ptr(),
                other_ptr = in(reg) other.bits.as_ptr(),
                index = inout(reg) index => _,
                out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
                out("xmm4") _, out("xmm5") _, out("xmm6") _, out("xmm7") _,
                out("xmm8") _, out("xmm9") _, out("xmm10") _, out("xmm11") _,
                out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
                options(nostack)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let self_ptr = self.bits.as_mut_ptr();
            let other_ptr = other.bits.as_ptr();
            asm!(
                "2:",
                "ldp q0, q1, [{self_ptr}]",
                "ldp q2, q3, [{self_ptr}, #32]",
                "ldp q4, q5, [{self_ptr}, #64]",
                "ldp q6, q7, [{self_ptr}, #96]",
                "ldp q8, q9, [{other_ptr}], #32",
                "ldp q10, q11, [{other_ptr}], #32",
                "ldp q12, q13, [{other_ptr}], #32",
                "ldp q14, q15, [{other_ptr}], #32",
                "and v0.16b, v0.16b, v8.16b",
                "and v1.16b, v1.16b, v9.16b",
                "and v2.16b, v2.16b, v10.16b",
                "and v3.16b, v3.16b, v11.16b",
                "and v4.16b, v4.16b, v12.16b",
                "and v5.16b, v5.16b, v13.16b",
                "and v6.16b, v6.16b, v14.16b",
                "and v7.16b, v7.16b, v15.16b",
                "stp q0, q1, [{self_ptr}], #32",
                "stp q2, q3, [{self_ptr}], #32",
                "stp q4, q5, [{self_ptr}], #32",
                "stp q6, q7, [{self_ptr}], #32",
                "subs {count:w}, {count:w}, #1",
                "b.ne 2b",
                self_ptr = inout(reg) self_ptr => _,
                other_ptr = inout(reg) other_ptr => _,
                count = inout(reg) 64 => _,
                out("q0") _, out("q1") _, out("q2") _, out("q3") _,
                out("q4") _, out("q5") _, out("q6") _, out("q7") _,
                out("q8") _, out("q9") _, out("q10") _, out("q11") _,
                out("q12") _, out("q13") _, out("q14") _, out("q15") _,
                options(nostack)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            for i in 0..1024 {
                self.bits[i] &= other.bits[i];
            }
        }
    }

    fn difference_with(&mut self, other: &Self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut index = 0;
            asm!(
                "2:",
                "movdqu xmm0, [{self_ptr} + {index}]",
                "movdqu xmm1, [{self_ptr} + {index} + 16]",
                "movdqu xmm2, [{self_ptr} + {index} + 32]",
                "movdqu xmm3, [{self_ptr} + {index} + 48]",
                "movdqu xmm4, [{self_ptr} + {index} + 64]",
                "movdqu xmm5, [{self_ptr} + {index} + 80]",
                "movdqu xmm6, [{self_ptr} + {index} + 96]",
                "movdqu xmm7, [{self_ptr} + {index} + 112]",
                "movdqu xmm8, [{other_ptr} + {index}]",
                "movdqu xmm9, [{other_ptr} + {index} + 16]",
                "movdqu xmm10, [{other_ptr} + {index} + 32]",
                "movdqu xmm11, [{other_ptr} + {index} + 48]",
                "movdqu xmm12, [{other_ptr} + {index} + 64]",
                "movdqu xmm13, [{other_ptr} + {index} + 80]",
                "movdqu xmm14, [{other_ptr} + {index} + 96]",
                "movdqu xmm15, [{other_ptr} + {index} + 112]",
                "pandn xmm8, xmm0",
                "pandn xmm9, xmm1",
                "pandn xmm10, xmm2",
                "pandn xmm11, xmm3",
                "pandn xmm12, xmm4",
                "pandn xmm13, xmm5",
                "pandn xmm14, xmm6",
                "pandn xmm15, xmm7",
                "movdqu [{self_ptr} + {index}], xmm8",
                "movdqu [{self_ptr} + {index} + 16], xmm9",
                "movdqu [{self_ptr} + {index} + 32], xmm10",
                "movdqu [{self_ptr} + {index} + 48], xmm11",
                "movdqu [{self_ptr} + {index} + 64], xmm12",
                "movdqu [{self_ptr} + {index} + 80], xmm13",
                "movdqu [{self_ptr} + {index} + 96], xmm14",
                "movdqu [{self_ptr} + {index} + 112], xmm15",
                "add {index}, 128",
                "cmp {index}, 8192",
                "jb 2b",
                self_ptr = in(reg) self.bits.as_mut_ptr(),
                other_ptr = in(reg) other.bits.as_ptr(),
                index = inout(reg) index => _,
                out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
                out("xmm4") _, out("xmm5") _, out("xmm6") _, out("xmm7") _,
                out("xmm8") _, out("xmm9") _, out("xmm10") _, out("xmm11") _,
                out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
                options(nostack)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let self_ptr = self.bits.as_mut_ptr();
            let other_ptr = other.bits.as_ptr();
            asm!(
                "2:",
                "ldp q0, q1, [{self_ptr}]",
                "ldp q2, q3, [{self_ptr}, #32]",
                "ldp q4, q5, [{self_ptr}, #64]",
                "ldp q6, q7, [{self_ptr}, #96]",
                "ldp q8, q9, [{other_ptr}], #32",
                "ldp q10, q11, [{other_ptr}], #32",
                "ldp q12, q13, [{other_ptr}], #32",
                "ldp q14, q15, [{other_ptr}], #32",
                "bic v0.16b, v0.16b, v8.16b",
                "bic v1.16b, v1.16b, v9.16b",
                "bic v2.16b, v2.16b, v10.16b",
                "bic v3.16b, v3.16b, v11.16b",
                "bic v4.16b, v4.16b, v12.16b",
                "bic v5.16b, v5.16b, v13.16b",
                "bic v6.16b, v6.16b, v14.16b",
                "bic v7.16b, v7.16b, v15.16b",
                "stp q0, q1, [{self_ptr}], #32",
                "stp q2, q3, [{self_ptr}], #32",
                "stp q4, q5, [{self_ptr}], #32",
                "stp q6, q7, [{self_ptr}], #32",
                "subs {count:w}, {count:w}, #1",
                "b.ne 2b",
                self_ptr = inout(reg) self_ptr => _,
                other_ptr = inout(reg) other_ptr => _,
                count = inout(reg) 64 => _,
                out("q0") _, out("q1") _, out("q2") _, out("q3") _,
                out("q4") _, out("q5") _, out("q6") _, out("q7") _,
                out("q8") _, out("q9") _, out("q10") _, out("q11") _,
                out("q12") _, out("q13") _, out("q14") _, out("q15") _,
                options(nostack)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            for i in 0..1024 {
                self.bits[i] &= !other.bits[i];
            }
        }
    }

    fn symmetric_difference_with(&mut self, other: &Self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::asm;
            let mut index = 0;
            asm!(
                "2:",
                "movdqu xmm0, [{self_ptr} + {index}]",
                "movdqu xmm1, [{self_ptr} + {index} + 16]",
                "movdqu xmm2, [{self_ptr} + {index} + 32]",
                "movdqu xmm3, [{self_ptr} + {index} + 48]",
                "movdqu xmm4, [{self_ptr} + {index} + 64]",
                "movdqu xmm5, [{self_ptr} + {index} + 80]",
                "movdqu xmm6, [{self_ptr} + {index} + 96]",
                "movdqu xmm7, [{self_ptr} + {index} + 112]",
                "movdqu xmm8, [{other_ptr} + {index}]",
                "movdqu xmm9, [{other_ptr} + {index} + 16]",
                "movdqu xmm10, [{other_ptr} + {index} + 32]",
                "movdqu xmm11, [{other_ptr} + {index} + 48]",
                "movdqu xmm12, [{other_ptr} + {index} + 64]",
                "movdqu xmm13, [{other_ptr} + {index} + 80]",
                "movdqu xmm14, [{other_ptr} + {index} + 96]",
                "movdqu xmm15, [{other_ptr} + {index} + 112]",
                "pxor xmm0, xmm8",
                "pxor xmm1, xmm9",
                "pxor xmm2, xmm10",
                "pxor xmm3, xmm11",
                "pxor xmm4, xmm12",
                "pxor xmm5, xmm13",
                "pxor xmm6, xmm14",
                "pxor xmm7, xmm15",
                "movdqu [{self_ptr} + {index}], xmm0",
                "movdqu [{self_ptr} + {index} + 16], xmm1",
                "movdqu [{self_ptr} + {index} + 32], xmm2",
                "movdqu [{self_ptr} + {index} + 48], xmm3",
                "movdqu [{self_ptr} + {index} + 64], xmm4",
                "movdqu [{self_ptr} + {index} + 80], xmm5",
                "movdqu [{self_ptr} + {index} + 96], xmm6",
                "movdqu [{self_ptr} + {index} + 112], xmm7",
                "add {index}, 128",
                "cmp {index}, 8192",
                "jb 2b",
                self_ptr = in(reg) self.bits.as_mut_ptr(),
                other_ptr = in(reg) other.bits.as_ptr(),
                index = inout(reg) index => _,
                out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _,
                out("xmm4") _, out("xmm5") _, out("xmm6") _, out("xmm7") _,
                out("xmm8") _, out("xmm9") _, out("xmm10") _, out("xmm11") _,
                out("xmm12") _, out("xmm13") _, out("xmm14") _, out("xmm15") _,
                options(nostack)
            );
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::asm;
            let self_ptr = self.bits.as_mut_ptr();
            let other_ptr = other.bits.as_ptr();
            asm!(
                "2:",
                "ldp q0, q1, [{self_ptr}]",
                "ldp q2, q3, [{self_ptr}, #32]",
                "ldp q4, q5, [{self_ptr}, #64]",
                "ldp q6, q7, [{self_ptr}, #96]",
                "ldp q8, q9, [{other_ptr}], #32",
                "ldp q10, q11, [{other_ptr}], #32",
                "ldp q12, q13, [{other_ptr}], #32",
                "ldp q14, q15, [{other_ptr}], #32",
                "eor v0.16b, v0.16b, v8.16b",
                "eor v1.16b, v1.16b, v9.16b",
                "eor v2.16b, v2.16b, v10.16b",
                "eor v3.16b, v3.16b, v11.16b",
                "eor v4.16b, v4.16b, v12.16b",
                "eor v5.16b, v5.16b, v13.16b",
                "eor v6.16b, v6.16b, v14.16b",
                "eor v7.16b, v7.16b, v15.16b",
                "stp q0, q1, [{self_ptr}], #32",
                "stp q2, q3, [{self_ptr}], #32",
                "stp q4, q5, [{self_ptr}], #32",
                "stp q6, q7, [{self_ptr}], #32",
                "subs {count:w}, {count:w}, #1",
                "b.ne 2b",
                self_ptr = inout(reg) self_ptr => _,
                other_ptr = inout(reg) other_ptr => _,
                count = inout(reg) 64 => _,
                out("q0") _, out("q1") _, out("q2") _, out("q3") _,
                out("q4") _, out("q5") _, out("q6") _, out("q7") _,
                out("q8") _, out("q9") _, out("q10") _, out("q11") _,
                out("q12") _, out("q13") _, out("q14") _, out("q15") _,
                options(nostack)
            );
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            for i in 0..1024 {
                self.bits[i] ^= other.bits[i];
            }
        }
    }
}
