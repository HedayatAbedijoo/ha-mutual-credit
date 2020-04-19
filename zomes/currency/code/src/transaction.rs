use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};
use std::convert::TryFrom;

use crate::attestation::Attestation;
use crate::utils::get_chain_agent_id;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub timestamp: usize,
    pub amount: usize,
    pub balance: usize,
    pub attestation_address: Address,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "transaction",
        description: "this is a transaction entry for each party source chain",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Transaction>| {
            // TODO
            // Create:
           // sender_address should equal to Agent_Address (Transaction Entry is Private you can write Agent_Address in validation),(what about holoport??? )
           // sender_address is not equal receiver_Address
           // Check Balance, by query source-chain.
           // balance should support amount
           /// Edit:
           /// Use can just update attestation_address one time. when it is empty
           /// Delete: not allowed
           Ok(())
        }
    )
}

//fn Send_Transaction(ammount,balance){

//1- create Transaction
//2- commit Transaction
//3- 5_last_tnx = get last 5 tranactions
//3 - bob_signature=  send_message_to_bob(transaction, 5_last_tnx )
//4- attensation_address = create attensation entry.  status: offered
//5 - create link from agent_address to attestation entry
//6- bob_confirmation_result = send_message_to_bob(attestation_address)
// 7- Return Ok, transaction offered properly
//}

// fn Recieve_Transaction(transaction, 5_last_tnx, sender_address){

// Validation Sender ledger using DHT data, and data sent by Sender:
// 5_last_tnx_dht = query DHT links from "sender_address" typeof "sender" or "receiver"
// if 5_last_tnx_dht != 5_last_tnx  return Error. validation false
// else. compare each hash_of_transaction with the source of transaction. 5_last_tnx_dht.hash_of_transaction == hash(5_last_tnx)
// Create local transaction.
// Sign the transaction and return signature

// }

// fn Finish_Transaction_HandShake(Attestation_Address,transaction){

// attestation = DHT.query(Attestation_Address)
// check that the Signature, Hash_Of_Transaction are correct with my local.
// if it is not OK, return Error, Change the status of Attestation to Rejected
// If it is OK, Create a link from my Agent_Address to Attestation
// Return OK
// }
