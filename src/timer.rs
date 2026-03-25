#[cfg(target_os = "macos")]
pub mod cpu_timer {
    use std::time::Duration;

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    fn get_cycles() -> u64 {
        unsafe {
            let cycles: u64;
            core::arch::asm!("mrs x0, cntvct_el0", out("x0") cycles);
            cycles
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    #[inline(always)]
    fn get_cycles() -> u64 {
        0
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    fn get_freq_hz() -> u64 {
        unsafe {
            let freq: u64;
            core::arch::asm!("mrs x0, cntfrq_el0", out("x0") freq);
            freq
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn get_freq_hz() -> u64 {
        24_000_000
    }

    pub struct CpuTimer {
        start: u64,
        freq_hz: u64,
    }

    impl CpuTimer {
        pub fn new() -> Self {
            CpuTimer {
                start: get_cycles(),
                freq_hz: get_freq_hz(),
            }
        }

        pub fn elapsed_cycles(&self) -> u64 {
            get_cycles().wrapping_sub(self.start)
        }

        pub fn reset(&mut self) {
            self.start = get_cycles();
        }
    }

    impl Default for CpuTimer {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub mod cpu_timer {
    use std::time::Duration;

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub fn get_cycles() -> u64 {
        unsafe { core::arch::x86_64::_rdtsc() }
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub fn get_cycles() -> u64 {
        unsafe {
            let mut cycles: u64;
            core::arch::aarch64::__asm!("mrs {0}, pmccntr_el0", out(reg) cycles);
            cycles
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    #[inline(always)]
    pub fn get_cycles() -> u64 {
        0
    }

    pub struct CpuTimer {
        start: u64,
    }

    impl CpuTimer {
        pub fn new() -> Self {
            CpuTimer {
                start: get_cycles(),
            }
        }

        pub fn elapsed_cycles(&self) -> u64 {
            get_cycles().wrapping_sub(self.start)
        }

        pub fn reset(&mut self) {
            self.start = get_cycles();
        }
    }

    impl Default for CpuTimer {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use cpu_timer::CpuTimer;
