use gstd::{msg, prelude::*, ActorId};
use primitive_types::U256;
pub type Payout = BTreeMap<ActorId, u128>;
use gear_lib::non_fungible_token::io::*;
use nft_io::*;
use crate::{TokenId, ContractId};

pub async fn nft_transfer(
    nft_program_id: ContractId,
    to: ActorId,
    token_id: TokenId,
    amount: u128,
) -> Payout {
    let response: Vec<u8> = msg::send_and_wait_for_reply(
        nft_program_id,
        NFTAction::TransferPayout {
            to,
            token_id,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("error in transfer");
    let decoded_response: NFTTransferPayout =
        NFTTransferPayout::decode(&mut &response[..]).expect("Error in decoding payouts");
    decoded_response.payouts
}

pub async fn nft_approve(nft_program_id: ContractId, to: ActorId, token_id: TokenId) {
    let _approve_response: Vec<u8> =
        msg::send_and_wait_for_reply(nft_program_id, NFTAction::Approve { to, token_id }, 0)
            .unwrap()
            .await
            .expect("error in transfer");
}
