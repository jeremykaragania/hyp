pub struct Gic {
    distributor_regs: *mut GicDistributorRegisters,
    cpu_interface_regs: *mut GicCpuInterfaceRegisters,
    // Even though this is a hypervisor we still don't assume that the GIC has
    // the Virtualization Extensions.
    virtual_interface_control_regs: Option<*mut GicVirtualInterfaceControlRegisters>,
    virtual_cpu_interface_regs: Option<*mut GicVirtualCpuInterfaceRegisters>,
}

impl Gic {
    pub fn new(
        distributor_base: u64,
        cpu_interface_base: u64,
        virtual_interface_control_base: Option<u64>,
        virtual_cpu_interface_base: Option<u64>,
    ) -> Self {
        Self {
            distributor_regs: distributor_base as *mut GicDistributorRegisters,
            cpu_interface_regs: cpu_interface_base as *mut GicCpuInterfaceRegisters,
            virtual_interface_control_regs: if let Some(addr) = virtual_interface_control_base {
                Some(addr as *mut GicVirtualInterfaceControlRegisters)
            } else {
                None
            },
            virtual_cpu_interface_regs: if let Some(addr) = virtual_cpu_interface_base {
                Some(addr as *mut GicVirtualCpuInterfaceRegisters)
            } else {
                None
            },
        }
    }
}

unsafe impl Send for Gic {}

#[repr(C)]
struct GicDistributorRegisters {
    ctlr: u32,                            // Distributor Control Register.
    typer: u32,                           // Interrupt Controller Type Register.
    iidr: u32,                            // Distributor Implementer Identification Register.
    _reserved_0: [u32; 5],                // Reserved.
    _implementation_defined_0: [u32; 8],  // Implementation Defined.
    _reserved_1: [u32; 16],               // Reserved.
    igroupr: [u32; 32],                   // Interrupt Group Registers.
    isenabler: [u32; 32],                 // Interrupt Set-Enable Registers.
    icenabler: [u32; 32],                 // Interrupt Clear-Enable Registers.
    ispendr: [u32; 32],                   // Interrupt Set-Pending Registers.
    icpendr: [u32; 32],                   // Interrupt Clear-Pending Registers.
    isactiver: [u32; 32],                 // GICv2 Interrupt Set-Active Registers.
    icactiver: [u32; 32],                 // Interrupt Clear-Active Registers.
    ipriorityr: [u32; 255],               // Interrupt Priority Registers.
    _reserved_2: u32,                     // Reserved.
    itargetsr: [u32; 255],                // Interrupt Processor Targets Registers.
    _reserved_3: u32,                     // Reserved.
    icfgr: [u32; 64],                     // Interrupt Configuration Registers.
    _implementation_defined_1: [u32; 64], // Implementation Defined.
    nsacr: [u32; 64],                     // Non-secure Access Control Registers, optional
    sgir: u32,                            // Software Generated Interrupt Register.
    _reserved_4: [u32; 3],                // Reserved.
    cpendsgir: [u32; 4],                  // SGI Clear-Pending Registers
    spendsgir: [u32; 4],                  // SGI Set-Pending Registers.
    _reserved_5: [u32; 40],               // Reserved.
    _implementation_defined_2: [u32; 12], // Implementation defined.
}

#[repr(C)]
struct GicCpuInterfaceRegisters {
    ctlr: u32,                            // CPU Interface Control Register.
    pmr: u32,                             // Interrupt Priority Mask Register.
    bpr: u32,                             // Binary Point Register.
    iar: u32,                             // Interrupt Acknowledge Register.
    eoir: u32,                            // End of Interrupt Register.
    rpr: u32,                             // Running Priority Register.
    hppir: u32,                           // Highest Priority Pending Interrupt Register.
    abpr: u32,                            // Aliased Binary Point Register.
    aiar: u32,                            // Aliased Interrupt Acknowledge Register,
    aeoir: u32,                           // Aliased End of Interrupt Register.
    ahppir: u32,                          // Aliased Highest Priority Pending Interrupt Register.
    _reserved_0: [u32; 5],                // Reserved.
    _implementation_defined_0: [u32; 36], // Implementation Defined.
    apr: [u32; 4],                        // Active Priorities Registers.
    nsapr: [u32; 4],                      // Non-secure Active Priorities Registers.
    _reserved_1: [u32; 3],                // Reserved.
    iidr: u32,                            // CPU Interface Identification Register.
    _reserved_2: [u32; 960],              // Reserved.
    dir: u32,                             // Deactivate Interrupt Register.
}

#[repr(C)]
pub struct GicVirtualInterfaceControlRegisters {
    hcr: u32,               // Hypervisor Control Register.
    vtr: u32,               // VGIC Type Register.
    vmcr: u32,              // Virtual Machine Control Register.
    _reserved_0: u32,       // Reserved.
    misr: u32,              // Maintenance Interrupt Status Register.
    _reserved_1: [u32; 3],  // Reserved.
    eisr: [u32; 2],         // End of Interrupt Status Registers 0 and 1
    _reserved_2: [u32; 2],  // Reserved.
    elsr: [u32; 2],         // Empty List Register Status Registers 0 and 1.
    _reserved_3: [u32; 46], // Reserved.
    apr: u32,               // Active Priorities Register.
    _reserved_4: [u32; 3],  // Reserved for GICH_APR1-GICH_APR3.
    lr: [u32; 64],          // List Registers 0-63, see GICH_LRn.
}

#[repr(C)]
pub struct GicVirtualCpuInterfaceRegisters {
    ctlr: u32,  // Virtual Machine Control Register.
    pmr: u32,   // VM Priority Mask Register.
    bpr: u32,   // VM Binary Point Register.
    iar: u32,   // VM Interrupt Acknowledge Register.
    eoir: u32,  // VM End of Interrupt Register.
    rpr: u32,   // VM Running Priority Register.
    hppir: u32, // VM Highest Priority Pending Interrupt Register.
    abpr: u32,  // VM Aliased Binary Point Register.
    aiar: u32,  // VM Aliased Interrupt Acknowledge Register.
    aeoir: u32, // VM Aliased End of Interrupt Register.
    ahppir: u32, /* VM Aliased Highest Priority Pending Interrupt
                 * Register. */
    _reserved_0: [u32; 5],                // Reserved.
    _implementation_defined_0: [u32; 36], // Implementation Defined.
    apr: [u32; 4],                        // VM Active Priorities Registers.
    _reserved_1: [u32; 4],                /* Reserved for second set of Active Priorities
                                           * Registers. */
    _reserved_2: [u32; 3],    // Reserved.
    iidr: u32,                // VM CPU Interface Identification Register.
    _reserved_3: [u32; 960],  // Reserved.
    dir: u32,                 // VM Deactivate Interrupt Register.
    _reserved_4: [u32; 1024], // Reserved.
}
