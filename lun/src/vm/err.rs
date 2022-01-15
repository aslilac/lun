/**
 * We have to make up our own Result type so that we can impl Default for it
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VmResult {
    Ok,
    Err(VmError),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VmError {
    BrOverflow,
    RegOverflow,
    StackOverflow,
    StackUnderflow,
}

impl Default for VmResult {
    fn default() -> Self {
        Self::Ok
    }
}
