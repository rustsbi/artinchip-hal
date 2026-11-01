//! On-flash layout shared by both boards.

use artinchip_hal::aic_upg::Partition;

include!(concat!(env!("OUT_DIR"), "/partitions.rs"));
