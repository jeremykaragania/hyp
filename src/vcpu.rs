pub struct VCpu {
    pub registers: VCpuRegisters,
}

// NOTE: The initial idea was to preserve all registers associated with any EL
// lower than EL2. But there are quite a few. We've decided to just start with a
// select few that aren't read only.
pub struct VCpuRegisters {
    // General purpose registers.
    general_purpose: [u64; 31], // X0-X30.

    // Special registers.
    xzr: u64,      // Zero register.
    pc: u64,       // Program counter.
    sp_el0: u64,   // Stack pointer (EL0).
    sp_el1: u64,   // Stack pointer (EL1).
    spsr_el1: u64, // Program Status Register (EL1).
    elr_el1: u64,  // Exception Link Register (EL1).

    // System registers.
    cntpct_el0: u64,    // Counter-timer Physical Count Register.
    cntkctl_el1: u64,   // Counter-timer Kernel Control Register.
    cntp_cval_el0: u64, // Counter-timer Physical Timer Compare Value Register.
    cpacr_el1: u64,     // Coprocessor Access Control Register.
    csselr_el1: u64,    // Cache Size Selection Register.
    cntp_ctl_el0: u64,  // Counter-timer Physical Control Register
    esr_el1: u64,       // Exception Syndrome Register (EL1).
    far_el1: u64,       // Fault Address Register.
    fpcr: u64,          // Floating-point Control Register.
    fpsr: u64,          // Floating-point Status Register.
    mair_el1: u64,      // Memory Attribute Indirection Register.
    sctlr_el0: u64,     // System Control Register (EL0).
    sctlr_el1: u64,     // System Control Register (EL1).
    tcr_el1: u64,       // Translation Control Register (EL1).
    tpidr_el0: u64,     // User Read/Write Thread ID Register (EL0).
    tpidr_el1: u64,     // User Read/Write Thread ID Register (EL1).
    ttbr0_el1: u64,     // Translation Table Base Register 0 (EL1).
    ttbr1_el1: u64,     // Translation Table Base Register 1 (EL1).
    vbar_el1: u64,      // Vector Based Address Register (EL1).
}
