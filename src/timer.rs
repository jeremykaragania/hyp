unsafe extern "C" {
    fn read_cntpct_el0() -> u64;
    fn read_cntp_ctl_el0() -> u64;
    fn write_cntp_ctl_el0(val: u64);
    fn write_cntp_cval_el0(val: u64);
}

pub fn init_timer() {
    // ENABLE = 1.
    let reg = 1;
    unsafe {
        write_cntp_ctl_el0(reg);
    };
}
