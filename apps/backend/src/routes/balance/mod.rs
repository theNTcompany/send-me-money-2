use crate::{Authorization, Balances};
use rocket::State;
use std::sync::atomic::Ordering;

#[derive(Responder)]
pub(crate) enum BalanceResponses {
    #[response(status = 200)]
    Ok(String),

    #[response(status = 400)]
    Unauthorized(String),
}

#[get("/balance")]
pub(crate) fn get_balance<'r>(balances: &State<Balances>, auth: Authorization) -> BalanceResponses {
    if !auth.value.eq("password123") {
        BalanceResponses::Unauthorized("".into())
    } else {
        BalanceResponses::Ok(balances.me.load(Ordering::Relaxed).to_string())
    }
}
