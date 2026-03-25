use core::time;
use std::thread::sleep;

use crate::timer::cpu_timer;

#[test]
fn test_chrono() {
    let timer = cpu_timer::CpuTimer::new();
    let time_ms = time::Duration::from_nanos(100);
    sleep(time_ms);
    assert!(timer.elapsed_cycles() > 0);
}

#[test]
fn test_reset_chrono() {
    let mut timer = cpu_timer::CpuTimer::new();
    let time_ms = time::Duration::from_nanos(100);
    sleep(time_ms);
    let t = timer.elapsed_cycles();
    timer.reset();
    sleep(time_ms * 100);
    assert!(t < timer.elapsed_cycles());
}
