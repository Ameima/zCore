// RISCV
// Linear mapping

#[cfg(feature = "board_qemu")]
pub const KERNEL_OFFSET: usize = 0xFFFF_FFFF_8000_0000;
#[cfg(feature = "board_qemu")]
pub const MEMORY_OFFSET: usize = 0x8000_0000;
#[cfg(feature = "board_qemu")]
pub const MEMORY_END: usize = 0x8800_0000; // TODO: get memory end from device tree

#[cfg(feature = "board_d1")]
pub const KERNEL_OFFSET: usize = 0xFFFFFFFF_C0000000;
#[cfg(feature = "board_d1")]
pub const MEMORY_OFFSET: usize = 0x40000000;
#[cfg(feature = "board_d1")]
pub const MEMORY_END: usize = 0x60000000; // 512M

pub const PHYSICAL_MEMORY_OFFSET: usize = KERNEL_OFFSET - MEMORY_OFFSET;

pub const MAX_DTB_SIZE: usize = 0x2000;
