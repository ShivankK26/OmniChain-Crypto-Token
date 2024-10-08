use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::{Deserialize, Serialize};
use cosmwasm_std::{Binary, Uint128};
use schemars::JsonSchema;


#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub chain_id: String,
    pub contract_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainTypeInfo {
    pub chain_id: String,
    pub chain_type: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_code_id: u64,
    pub token_name: String,
    pub token_symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetWhiteListedContracts {
        contracts: Vec<ContractInfo>,
    },
    SetChainId {
        id: String,
    },
    SetXerc20Addr {
        addr: String,
    },
    Mint {
        recipient: String,
        amount: Uint128,
    },
    UpdateOwner {
        new_owner: String,
    },
    SetChainTypes {
        chain_type_info: Vec<ChainTypeInfo>,
    },
    TrasferCrossChain {
        amount: Uint128,
        recipient: Binary,
        dest_chain_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    FetchOwner {},
    FetchXerc20 {},
    FetchChainId {},
    FetchChainType { chain_id: String },
    FetchWhiteListedContract { chain_id: String },
    AllWhiteListedContract {},
}