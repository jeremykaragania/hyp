unsafe extern "C" {
    fn read_cntpct_el0() -> u64;
    fn read_cntp_ctl_el0() -> u64;
    fn write_cntp_ctl_el0(val: u64);
    fn write_cntp_cval_el0(val: u64);
}
