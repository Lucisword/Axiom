use anchor_lang::prelude::*;

#[error_code]
pub enum AxiomError {
    #[msg("Capability authority is not active")]
    CapabilityNotActive,

    #[msg("Action exceeds the capability per-action limit")]
    ExceedsPerActionLimit,
}
