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
pub enum Status {
    Offered,
    Confirmed,
    Rejected,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Attestation {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub timestamp: usize,
    pub hash_of_transaction: String,
    pub sender_signature: String,
    pub receiver_signature: String,
    pub transaction_status: Status,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "attestation",
        description: "this is a attestation entry for confimation of a transaction",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Attestation>| {
            // TODO
            // Create:
            // Query DHT by content of Entry. there shouldn't be the same Attestation in DHT
            //Status should be Offered
           // sender_address should equal operator_creator in the Header
           // sender_address is not equal receiver_Address
           // HashOfTransaction not null
           // SenderSignature is not null
           /// Receiver Signature is not Null
           /// Edit:
           /// sender can not do any change
           /// Just allowed for reciever_address. just onces, change the status from Offered to Confrim
           /// Delete: not allowed
           Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "offer",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                  // TODO:
                  // Create:  base_address should be equal to sender_address
                  // Edit: not allowed.
                  // Delete: not allowed
                  Ok(())
                }
            ),
            from!(
              "%agent_id",
              link_type: "receiver",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              }              ,
              validation: | _validation_data: hdk::LinkValidationData | {
                  // TODO:
                  // Create: base_address should be equal to receiver_address
                  // Edit: not allowed.
                  // Delete: not allowed
                 Ok(())
              }
          )
        ]
    )
}

// fn  Confirm_Or_Reject_Transaction(attestation_address){
// attestation = DHT.query()
// result { last_5_tnx_attestation_address, transaction } = send_message_get_your_last_5_transaction();
// double check DHT for the last 5 attestation and  last_5_tnx_attestation_address.
// if still the balance can cover amount, update attestation Status = Confirm
//  else Update attestation Status = Reject
//}
