use crate::bitmap_words;
use crate::mm::pool::Pool;
use crate::sync::Spinlock;
use crate::vcpu::VCpu;

const MAX_VMS: usize = 8;
const MAX_VM_VCPUS: usize = 4;

// This is the only place where we get VMs from.
pub static VMS: Spinlock<Pool<Vm, MAX_VMS, { bitmap_words!(MAX_VMS) }>> =
    Spinlock::new(Pool::new());

// TODO: How can we easily enforce that `vcpu_count` < MAX_VM_VCPUS?
struct Vm {
    id: u64, // We might just set this to the `PoolIndex` from `VMS`.
    cpus: [VCpu; MAX_VM_VCPUS],
    vcpu_count: usize,
}
