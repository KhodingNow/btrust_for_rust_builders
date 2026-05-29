
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("ZaruPay111111111111111111111111111111");

pub const ZARU_MINT: Pubkey = pubkey!("TODO_REPLACE_WITH_REAL_ZARU_MINT");

#[program]
pub mod zaru_payments {

	use super::*;


	// 1. Merchant creates an invoice (PDA)

	pub fn create_invoice(

	ctx: Context<CreateInvoice>,
	amount: i64,
	expires_at: i64,

) -> Result<()> {

	let invoice = &mut ctx.accounts.invoice;
	invoice.merchant = ctx.accounts.merchants.key();
	invoice.payer = ctx.accounts.payer.key();
	invoice.amount = amount;
	invoice.zaru_mint = ctx.accounts.zaru_mint.key();
	invoice.paid = false;
	invoice.expires_at = expires_at;
	Ok(())

	}

// 2. Payer pays the invoice (direct SPL via CPI)
pub fn pay_invoice(ctx: Context<PayInvoice>) -> Result<()> {
	let invoice = &mut ctx.accounts.invoice;
	require!(!invoice.paid, ErrorCode::AlreadyPAid);
	require!(Clock::get()?.unix_timestamp < invoice.expires_at, ErrorCode::Expired);

	
	let cpi_accounts = Transfer {

	from: ctx.accounts.payer_token_account.to_account_info(),
	
	to: ctx.accounts.merchant_token_account.to_account_info(),
	
	authority: ctx.accounts.payer.to_account_info(),
};

let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

token::transfer(cpi_ctx, invoice.amount)?;


invoice.paid = true;
Ok(())
	
}

// Merchant cnacels unpaid invoice

pub fn cancel_invoice(_ctx: Context<CancelInvoice>) -> Result<()> {

	Ok(()) // PDA will be close automatically by Anchor

	}
}


#[derive(Accounts)]
pub struct CreateInvoice<'info> {
	#[account(init, payer = merchant, space = 8 + 32 + 8 + 32 + 1 + 8, 
	seeds = [b"invoice", merchant.key().as_ref(), payer.key().as_ref()], bump)]
	pub invoice: Account<'info, Invoice>,
	#[account(mut)]
	pub merchant: Signer<'info>,

	/// CHECK: any payer (can be anonymous)
	pub payer: AccountInfo<'info>,
	pub zaru_mint: Account<'info, token::Mint>, // ZARU mint
	pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayInvoice<'info> {
	#[account(mut, has_one = merchant, has_one = payer, constraint = 
	invoice.zaru_mint == zaru_mint.key())]	
	pub invoice: Account<'info, Invoice>,
	
	#[account(mut)]	
	pub payer: Signer<'info>,
	
	#[account(mut, constraint = payer_token_account.mint == invoice.zaru_mint)]
	pub payer_token_account: Account<'info, TokenAccount>,
	
	#[account(mut, constraint = merchant_token_account.mint == invoice.zaru_mint)]
	pub merchant_token_account: Account<'info, TokenAccount>,
	pub zaru_mint: Account<'info, token::Mint>,
	pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
pub struct CancelInvoice<'info> {
	#[account(mut, close = merchant, has_one = merchant)]
	pub invoice: Account<'info, Invoice>,
	pub merchant: Signer<'info>,
}


#[account]
pub struct Invoice {

	pub merchant: Pubkey,
	pub payer: Pubkey,
	pub amount: u64,
	pub zaru_mint: Pubkey,
	pub paid: bool,
	pub expires_at: i64,
}

#[error_code]
pub enum ErrorCode {

	#[msg("Invoice already paid")]
	AlreadyPaid,
	#[msg("Invoice expired")]
	Expired,
}























