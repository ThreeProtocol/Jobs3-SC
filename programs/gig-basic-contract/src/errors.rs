use anchor_lang::prelude::*;

#[error_code]
pub enum GigContractError {
    #[msg("Invalid seller is trying to release funds!")]
    InvalidSeller,
    #[msg("Invalid seller is trying to activate contract!")]
    InvalidActivator,
    #[msg("Invalid buyer is trying to accept contract!")]
    InvalidAcceptor,
    #[msg("Invalid buyer is trying to release funds!")]
    InvalidBuyer,
    #[msg("Invalid admin is trying to release funds!")]
    InvalidAdmin,
    #[msg("Dispute Amount should be 50 cent!")]
    InvalidDisputeAmount,
    #[msg("Contract is not active yet or already completed!")]
    CantRelease,
    #[msg("Contract status should be Created to accept!")]
    CantAccept,
    #[msg("Can not activate contract!")]
    CantActivate,
    #[msg("Contract is not pending or disputed yet so admin can't approve now or already completed!")]
    NotReadyYet,
    #[msg("Invalid payment token!")]
    PayTokenMintError,
    #[msg("Invalid pay amount!")]
    PayAmountError,
    #[msg("Hourly Contract was paused!")]
    HourlyContractPaused,
    #[msg("Hourly Contract was ended!")]
    HourlyContractEnded,
    #[msg("Exceeded weekly hours limit!")]
    WeeklyHoursLimitError,
    #[msg("It needs to be ready to pay!")]
    HourlyGigPayError,
    #[msg("Hourly Contract was not paid yet!")]
    HourlyContractNotPaidYet,
    #[msg("Invalid featured day!")]
    InvalidFeaturedDay,
}