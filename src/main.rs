use tokio::runtime::{Handle};
use tracing::{trace, log::Log};
use xavax_avalanche::{
    api::info,
    api::network::{node::{XChain, PChain}, node::Node},
    pvm::{tx_format::{CreateSubnetTx, SECP256K1OutputOwnersOutput, SignedTransaction, Transactions}, self},
    avm::tx_format::BaseTx, crypto::{avalanche::{keychain::DerivationPath, tx_signer::TxSigner}, engine::{elliptic_curves::secp256k1_sign_rsv, hash_functions::get_sha256_hash}},
    avm::tx_format::{TransferableOutput, Outputs},
    avm::tx_format::{SECP256K1TransferOutput, Credential, TransferableInput, SECP256K1TransferInput}, encoding::cb58::decode_cb58, parser::parser_traits::Parser
};

use console::Style;

mod cli;
use cli::custom_theme::XavaxTheme;

use crate::cli::commands::{entry_command::init_cli};


fn main() {

    let mut seed: String = "".to_string();
    let pink = Style::new().color256(197);

    

    println!("\n{}\n\nA simple cli tool that allows easy creation of a subnet or chain/custom VM to a subnet made using the xavax-avalanche SDK, check it out on crates.io!
Make sure to create an account before and save the seed phrase generated in a safe place! \n", pink.apply_to("▲ snvm-tool by xavax.io ▲"));

    

    println!("{}", pink.apply_to("________________________________________________________________________________ \n"));

    init_cli();
    
    
    /* TODO: delete this mess when the tool is fully functional

    let args = Args::parse();
    match args.startcmd {
        StartCmds::AddChain(arg) => {
            
        }
        StartCmds::CreateSubnet(arg) => {

        }
        StartCmds::CreateAccount(arg) => {

        }
    }
    
    trace!("Creating Async runtime");

    //Create the tokio async runtime.
    let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

    //Get the handle for the async tokio runtime
    let async_rt_handle: &Handle = rt.handle();

    let node: Node = Node{
        node_ip: "api.avax-test.network".to_string(),
        port: 443,
    };
    let pchain: PChain = PChain::default();
    async_rt_handle.spawn(async move {
        let cyan = Style::new().cyan();
        let resp = info::get_tx_fee(node).await.expect("Failure");
        println!("create subnet fee: {:?}", cyan.apply_to(resp.create_subnet_fee));

        let mut path: DerivationPath = DerivationPath::generate_path(None, 0, 0, 0);

        //don't use this seed-phrase anywhere, obviously, it's for testing purposes :D
        let key = xavax_avalanche::crypto::avalanche::keychain::Keys::get_key(
             "pass nation exit person change oxygen sword plastic cup vintage tornado rifle truth cloud keep able siege awkward about key enjoy palm radio section".to_string(),
             "fuji",
             path);
    
        let tx: CreateSubnetTx = CreateSubnetTx {
            base_tx: BaseTx {
                type_id: 0x10,
                network_id: pchain.blockchain_data.network_id,
                blockchain_id: pchain.blockchain_data.blockchain_id,
                outputs: vec![
                TransferableOutput {
                    asset_id: decode_cb58("U8iRqJoiJm8xZHAacmvYyZVwqQx6uDNtQeP3CQ6fcgQk3JqnK".to_string()).try_into().expect("Error!"),
                    output: Outputs::SECP256K1TransferOutput(SECP256K1TransferOutput{
                            type_id: 7,
                            amount: 1000000000 - resp.create_subnet_fee,
                            locktime: 0,
                            threshhold: 1,
                            addresses: vec![key.address.clone()]
                        })
                    } 
                ],
                inputs: vec![
                    TransferableInput{
                        output_consumer_owners: vec![key.address.clone()],
                        tx_id: decode_cb58("2eWRw85KwxsdWSFFcQxUQDCyPjMZHHGUtQMnh2FAtQbV7u7LfZ".to_string()).try_into().expect("Error!"),
                        utxo_index: 0,
                        asset_id: decode_cb58("U8iRqJoiJm8xZHAacmvYyZVwqQx6uDNtQeP3CQ6fcgQk3JqnK".to_string()).try_into().expect("Error!"),
                        input: SECP256K1TransferInput{
                            type_id: 5,
                            amount: 1000000000,
                            address_indices: vec![0]
                         }
                    }
                ],
                memo: vec![],
            },
            rewards_owner: SECP256K1OutputOwnersOutput {
                type_id: 11,
                locktime: 0,
                threshold: 1,
                addresses: vec![key.address.clone()]
            },
        };

        //println!("{:?}", tx.clone());
        let signed_tx = pvm::tx_format::Transactions::CreateSubnetTx(tx).sign_tx(&vec![key]).expect("Error: ");
    
        let tx_cb58: String = signed_tx;
        println!("{}", tx_cb58);
        
        let mut test_decode: SignedTransaction = SignedTransaction::default();
        test_decode.from_bytes(
        &decode_cb58("11111hBz4MxgVvQhpP4jdAW5V7NYimxRZJ737N374mGgsPiLeYDQNSdMj8o8MARVwN8HZJKCjKAMf3XQpES3yLBa4baZZPpVHnFP7LN8f7gFFjRMn3P3T1E15LbVdP9YYQzeQS3djQQS8RttVhdte6MRWu2XvWb2MFBcqPnzq4e8cZfFzNvzs2BhKCfgAtXbM3efCjvqZq81B7NW5tDfXdSbVCJz7dAFprSGdPNXVsLoN3iLLAQp3wMjuDzmAPcEEmri9F3X8VR2W2wuA49SnLkGiaWmuL7hqkMsooTeC8jjv7cEY7fwH48sQkZivAKBc4G2wJjh2cPd1xzGXdDPKh5BMcQpDDjc7aRMog7RBjBXaoJvKoHFbWjAnEwMwoW5Ss6PGGNbwKfF7bAKTuFataWKfF3YtvU2oR1cK75QWNyaSD8btVd1eS7ekukEeaVRmGLmFXAaHy5NiYMNFh".to_string()),
        None);
        
        //println!("{:#?}", test_decode);
    });


    loop {

    }
    */
}