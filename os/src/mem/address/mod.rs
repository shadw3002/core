


mod physaddr;
mod physpagenum;
mod virtaddr;
mod virtpagenum;
mod range;

pub use physaddr::PhysAddr;
pub use physpagenum::PhysPageNum;
pub use virtaddr::VirtAddr;
pub use virtpagenum::VirtPageNum;
pub use range::{VPNRange, SimpleRange, SimpleRangeIterator, StepByOne};

use super::PageTableEntry;
use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use core::fmt::{self, Debug, Formatter};

/// physical address
const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;