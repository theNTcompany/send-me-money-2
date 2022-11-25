use crate::{Authorization, Balances, Flag};
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use std::sync::atomic::Ordering;

#[derive(Responder)]
pub(crate) enum SendResponses {
    #[response(status = 200)]
    Ok(String),

    #[response(status = 400)]
    Unauthorized(String),
}

#[derive(Deserialize)]
pub(crate) struct SendBody {
    amount: i32,
}

#[post("/send", data = "<amount>")]
pub(crate) fn send<'r>(
    amount: Json<SendBody>,
    auth: Authorization,
    balances: &State<Balances>,
    flag: &State<Flag>,
) -> SendResponses {
    if !auth.value.eq("password123") {
        return SendResponses::Unauthorized("".into());
    }

    let amount = amount.amount;

    let my_bal = balances.me.load(Ordering::Relaxed);
    let bobs_bal = balances.bob.load(Ordering::Relaxed);

    balances.me.store(my_bal - amount, Ordering::Relaxed);
    balances.bob.store(bobs_bal + amount, Ordering::Relaxed);

    if my_bal - amount >= 1000000 {
        return SendResponses::Ok(flag.flag.clone());
    }

    SendResponses::Ok(balances.me.load(Ordering::Relaxed).to_string())
}
