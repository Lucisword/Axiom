use anchor_lang::prelude::*;

use crate::{
    constants::AUTHORITY_SEED,
    error::AxiomError,
    state::{AuthorityState, CapabilityAuthority},
};

#[derive(Accounts)]
#[instruction(capability: [u8; 32])]
pub struct AuthorizeAction<'info> {
    pub agent: Signer<'info>,

    #[account(
        seeds = [
            AUTHORITY_SEED,
            agent.key().as_ref(),
            capability.as_ref(),
        ],
        bump,
        has_one = agent,
    )]
    pub authority: Account<'info, CapabilityAuthority>,
}

pub fn handle_authorize_action(
    ctx: Context<AuthorizeAction>,
    _capability: [u8; 32],
    amount: u64,
) -> Result<()> {
    let authority = &ctx.accounts.authority;

    require!(
        authority.state == AuthorityState::Active,
        AxiomError::CapabilityNotActive
    );

    require!(
        amount <= authority.max_per_action,
        AxiomError::ExceedsPerActionLimit
    );

    Ok(())
}
