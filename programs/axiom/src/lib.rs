pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9x3boz9bPvwL1zfTMyDo3Qjv441TqCy7hQsjf2aWoW7x");

#[program]
pub mod axiom {
    use super::*;

    pub fn create_capability_authority(
    ctx: Context<CreateCapabilityAuthority>,
    capability: [u8; 32],
    max_per_action: u64,
) -> Result<()> {
    crate::instructions::create_capability_authority::handle_create_capability_authority(
        ctx,
        capability,
        max_per_action,
    )
}

pub fn authorize_action(
    ctx: Context<AuthorizeAction>,
    capability: [u8; 32],
    amount: u64,
) -> Result<()> {
    crate::instructions::authorize_action::handle_authorize_action(
        ctx,
        capability,
        amount,
    )
}
}
