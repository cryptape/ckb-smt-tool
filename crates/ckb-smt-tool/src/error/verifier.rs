//! Errors for verification.

#[repr(i8)]
pub enum UpdateError {
    ComputeOldRoot = 0x01,
    ComputeNewRoot,
    MismatchedOldRoot,
    MismatchedNewRoot,
    // This is not an error, just make sure the error code is less than 16.
    Unreachable = 0x10,
}

#[repr(i8)]
pub enum VerifyError {
    ComputeRoot = 0x01,
    MismatchedRoot,
    // This is not an error, just make sure the error code is less than 16.
    Unreachable = 0x10,
}
