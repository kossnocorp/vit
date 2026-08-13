use crate::prelude::*;

mod initializing;
pub use initializing::*;

pub enum VitState {
    Initializing(VitStateInitializing),
}
