use std::{env, fs};
use std::fmt::Error;
use std::str::FromStr;
use near_units::parse_near;
use serde::Deserialize;
use serde_json::json;
use workspaces::{Account, AccountId, Contract};

#[derive(Deserialize)]
pub struct GetSafesByOwner {
    safes: Vec<AccountId>,
}

async fn init() -> Result<(Contract, Account, Vec<u8>), Error> {
;
    let wasm = include_bytes!("../../target/wasm32-unknown-unknown/release/near_safe_factory.wasm");
    let worker = workspaces::sandbox().await.unwrap();

    let account = worker
        .dev_create_account()
        .await
        .unwrap();

    let contract = account.deploy(wasm)
        .await
        .unwrap()
        .into_result()
        .unwrap();
    let alice = account
        .create_subaccount( "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await.unwrap()
        .into_result().unwrap();

    alice
        .call(contract.id(), "init")
        .transact()
        .await
        .unwrap()
        .into_result()
        .unwrap();



    let wasm_filepath = fs::canonicalize(env::current_dir().unwrap().join("../code/hello_near.wasm")).unwrap();
    let wasm = fs::read(wasm_filepath).unwrap();

    Ok((contract, alice, wasm))
}


#[tokio::main]
async fn main() {
}

#[tokio::test]
async fn it_can_deploy_safe_contract() {
    let (contract, alice, wasm) = init().await.unwrap();
    let res = alice
        .call(contract.id(), "create_safe")
        .args_json(json!({
           "name": "maha",
            "code": wasm
        }))
        .max_gas()
        .transact()
        .await;

    println!("{:?}", res.unwrap());

    // assert!(res.unwrap().is_success());
    println!("{:?}", alice.id());

    let res = alice
        .call(contract.id(), "get_safes_by_owner")
        .args_json(json!({
            "owner_id": alice.id()
        })).transact()
        .await.unwrap();

    println!("{:?}", res);
}

#[ignore]
#[tokio::test]
async fn it_can_only_be_called_by_owner() {
    let (contract, _, wasm) = init().await.unwrap();
    let worker = workspaces::sandbox().await.unwrap();
    let (_, sk) = worker.dev_generate().await;
    let account = worker
        .create_tla("yash.test.near".parse().unwrap(), sk)
        .await
        .unwrap()
        .into_result()
        .unwrap();

    let res = account
        .call(contract.id(), "create_safe")
        .args_json(json!({
           "name": "maha",
            "code": wasm
        }))
        .max_gas()
        .transact()
        .await.unwrap();

    assert!(res.is_failure());
}

#[ignore]
#[tokio::test]
async fn it_can_call_deployed_contract() {
    let (contract, alice, wasm) = init().await.unwrap();
    let _ = alice
        .call(contract.id(), "create_safe")
        .args_json(json!({
           "name": "maha",
            "code": wasm
        }))
        .max_gas()
        .transact()
        .await.unwrap()
        .unwrap();

    let base_account: &str = alice.id().split(".").collect::<Vec<&str>>().get(1).unwrap();
    let contract_id = AccountId::from_str(format!("maha.{}", base_account).as_str()).unwrap();

    let res = alice
        .call(&contract_id, "get_greeting")
        .args_json(json!({
        }))
        .max_gas()
        .transact()
        .await.unwrap();

    assert!(res.is_success());

}
