use crate::sync::Spinlock;
use core::fmt;
use core::ptr::{read_volatile, write_volatile};

const DR_DATA: u32 = 255 << 0;

const FR_TXFF: u32 = 1 << 5;
const FR_RXFE: u32 = 1 << 4;
const FR_BUSY: u32 = 1 << 3;

const LCR_H_WLEN: u32 = 3 << 5;
const LCR_H_FEN: u32 = 1 << 4;
const LCR_H_STP2: u32 = 1 << 3;

const CR_UARTEN: u32 = 1 << 0;

pub static UART: Spinlock<PL011> = Spinlock::new(PL011::new(0x9000000));

#[repr(C)]
struct PL011Registers {
    dr: u32,
    rsr_ecr: u32,
    reserved_0: [u32; 4],
    fr: u32,
    reserved_1: u32,
    ilpr: u32,
    ibrd: u32,
    fbrd: u32,
    lcr_h: u32,
    cr: u32,
    ifls: u32,
    imsc: u32,
    ris: u32,
    mis: u32,
    icr: u32,
    dmacr: u32,
}

pub struct PL011 {
    regs: *mut PL011Registers,
}

impl PL011 {
    pub const fn new(base: u64) -> Self {
        Self {
            regs: base as *mut PL011Registers,
        }
    }

    pub fn init(&self) {
        let mut reg: u32;

        // Disable the UART.
        unsafe {
            reg = read_volatile(&mut (*self.regs).cr);
            reg &= !CR_UARTEN;
            write_volatile(&mut (*self.regs).cr, reg);
        }

        // Wait for the end of transmission or reception of the current
        // character.
        while unsafe { read_volatile(&mut (*self.regs).fr) } & FR_BUSY != 0 {}

        unsafe {
            reg = read_volatile(&mut (*self.regs).lcr_h);
            reg &= !LCR_H_FEN;
            write_volatile(&mut (*self.regs).lcr_h, reg);

            // The baud rate divisor is represented as a 22-bit number with a
            // 16-bit integer and 6-bit fractional part. The below values are
            // for a baud rate of 460800.
            write_volatile(&mut (*self.regs).ibrd, 3);
            write_volatile(&mut (*self.regs).fbrd, 16);

            reg = read_volatile(&mut (*self.regs).lcr_h);
            reg |= LCR_H_FEN;
            reg |= LCR_H_WLEN;
            reg &= !LCR_H_STP2;
            write_volatile(&mut (*self.regs).lcr_h, reg);

            reg = read_volatile(&mut (*self.regs).cr);
            reg |= CR_UARTEN;
            write_volatile(&mut (*self.regs).cr, reg);
        }
    }

    pub fn putchar(&self, c: u32) -> u32 {
        while unsafe { read_volatile(&mut (*self.regs).fr) } & FR_TXFF != 0 {}
        unsafe {
            write_volatile(&mut (*self.regs).dr, c);
        }
        c
    }

    pub fn getchar(&self) -> Result<u32, ()> {
        if unsafe { read_volatile(&mut (*self.regs).fr) } & FR_RXFE != 0 {
            return Err(());
        }

        Ok(unsafe { read_volatile(&mut (*self.regs).dr) & DR_DATA })
    }
}

unsafe impl Send for PL011 {}

impl fmt::Write for PL011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.putchar(c as u32);
        }
        Ok(())
    }
}
