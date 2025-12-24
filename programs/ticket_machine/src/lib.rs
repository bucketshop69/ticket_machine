use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
declare_id!("BhJXdCKNpLkRf2qWPwFYgYZFFE1WLngKNjMMhJBE7oku");

#[program]
pub mod ticket_machine {
    use anchor_lang::accounts::signer;

    use super::*;

    pub fn initialize_ticket_machine(ctx: Context<InitializeTicketMachine>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let ticket_machine = &mut ctx.accounts.ticket_machine;
        let signer = &mut ctx.accounts.signer;
        // set the admin
        ticket_machine.admin = signer.key();

        // make the ticket price to 0
        ticket_machine.ticket_price = 0;
        // set the tickets numbers to 0
        ticket_machine.ticket_number = 0;
        // make total earnings to 0
        ticket_machine.total_earnings = 0;

        Ok(())
    }

    pub fn set_price(ctx: Context<SetPrice>, price: u64) -> Result<()> {
        let ticket_machine = &mut ctx.accounts.ticket_machine;

        ticket_machine.ticket_price = price;
        Ok(())
    }

    pub fn withdraw_earnings(ctx: Context<WithdrawEarnings>, amount: u64) -> Result<()> {
        let ticket_machine = &mut ctx.accounts.ticket_machine;
        require!(
            ctx.accounts.signer.key() == ticket_machine.admin,
            MyError::Unauthorized
        );

        //check rent
        let rent = Rent::get()?.minimum_balance(8 + 32 + 8);
        // balance = total lamports - rent
        let balance = ticket_machine.to_account_info().lamports() - rent;
        require!(amount <= balance, MyError::InsufficientFunds);

        **ctx
            .accounts
            .ticket_machine
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;

        **ctx
            .accounts
            .signer
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, admin: Pubkey) -> Result<()> {
        let (expected_pda, _bump) =
            Pubkey::find_program_address(&[b"ticket_machine", admin.as_ref()], ctx.program_id);

        require!(
            ctx.accounts.ticket_machine.key() == expected_pda,
            MyError::Unauthorized
        );

        let ticket_machine = &mut ctx.accounts.ticket_machine;

        require!(ticket_machine.ticket_price > 0, MyError::GeneralError);

        let signer = &mut ctx.accounts.signer;

        let from_pubkey = signer.to_account_info();
        let to_pubkey = ticket_machine.to_account_info();

        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, ticket_machine.ticket_price)?;

        ticket_machine.total_earnings += ticket_machine.ticket_price;

        ticket_machine.ticket_number += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    ticket_machine: Account<'info, TicketMachine>,
    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawEarnings<'info> {
    #[account(
        mut,
        seeds = [b"ticket_machine", signer.key().as_ref()],
        bump
    )]
    ticket_machine: Account<'info, TicketMachine>,
    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPrice<'info> {
    #[account(
        mut,
        seeds = [b"ticket_machine", signer.key().as_ref()],
        bump
    )]
    ticket_machine: Account<'info, TicketMachine>,
    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeTicketMachine<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"ticket_machine", signer.key().as_ref()],
        bump
    )]
    ticket_machine: Account<'info, TicketMachine>,

    system_program: Program<'info, System>,
}

#[account]
pub struct TicketMachine {
    admin: Pubkey,       // 32
    ticket_price: u64,   // 8
    ticket_number: u64,  // 8
    total_earnings: u64, // 8
}

#[error_code]
pub enum MyError {
    #[msg("You are not authorized")]
    Unauthorized,

    #[msg("Insufficient funds in vault")]
    InsufficientFunds,

    #[msg("Error contact the owner")]
    GeneralError,
}
