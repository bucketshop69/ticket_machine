# Project 3: The Ticket Machine

## What We're Building

A ticket dispenser. Users pay SOL to get a ticket number. Admin can set ticket price and withdraw earnings.

## What We're Learning

- Admin/authority patterns
- Fee collection
- Sequential counters (ticket numbers)
- Multiple instruction types
- Role-based access control

## Acceptance Criteria

- Admin can call initialize to create ticket machine (sets price)
- Admin can call set_price to update ticket price
- User can call buy_ticket (pays SOL, gets ticket number)
- Ticket numbers increment (1, 2, 3...)
- Admin can call withdraw_earnings to collect fees
- Only admin can set price and withdraw
- Machine tracks total tickets sold and total earnings
- All tests pass
