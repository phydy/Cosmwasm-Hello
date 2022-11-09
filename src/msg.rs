use crate::state::Hello;
use cosmwasm_schema::{cw_serde, QueryResponses};
#[cw_serde]
pub struct InstantiateMsg {
    pub first_message: Hello,
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeGreeting { new_greeting: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Hello)]
    GetGreeting {},
}
