use super::entry_command::ApproveDeny;
use crate::cli::custom_theme::XavaxTheme;
use console::Style;
use dialoguer::{Select, Input, MultiSelect};

use xavax_avalanche::{crypto::{utils::validate_mnemonic, engine::{
        elliptic_curves::{MnemonicKeypair, SECP256K1Keypair}
    },
    avalanche::{keychain::DerivationPath, keychain::{Keys, encode_bech32_address}}
}, primitives::address::Address, api::network::node::PChain};



/** ## create_subnet();
`create_subnet()` Will initiate a command loop that prompts the user with options they can choose to create
a subnet; this mainly includes which addresses *(if any at all)* derived by the seed phrase should be used as authority keys, the signature
of these addresses will be required to add validators.

This will also include the address the user will have to send funds to in order to cover the tx fees the creation of a subnet requires.

At the end of subnet creation, a file will be created by the tool that stores all information about the subnet.
### Parameters:
* `seed_phrase` A string containing the mnemonic phrase.
*/
pub fn create_subnet(seed_phrase: &String) -> Result<ApproveDeny, String> {
    let theme = XavaxTheme::default();
    let pink = Style::new().color256(197);
    let pink_bold = Style::new().bold().color256(197);
    //let path = DerivationPath::generate_path(Some(9000), 0, 0, 0);

    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    println!("\n\nA set of keypairs will be derived from your seed-phrase using bip44 derivation paths. You will need to select: \n");

    println!("{} Which keypair will fund the transaction fee [You will need to send AVAX to this address]\n", pink_bold.apply_to("·"));
    println!("{} A list of keypairs that validators will require threshold signatures from in order to add a validator to the subnet, this
is only required if you don't want your subnet to be open for random validators.\n", pink_bold.apply_to("·"));

    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    //MnemonicKeypair::generate_keypair(seed_phrase, derivation_path)

    let multi_selected = &[
        "Yes",
        "No",
    ];

    let selection = Select::with_theme(&theme)
    .with_prompt("Would you like to proceed?")
    .default(0)
    .items(&multi_selected[..])
    .interact()
    .unwrap();

    let mut subnet_creation_decision: ApproveDeny;

    match selection {
        0 => {
            subnet_creation_decision = ApproveDeny::Approve;
        }
        1 => {
            subnet_creation_decision = ApproveDeny::Deny;
        }
        _ => {
            subnet_creation_decision = ApproveDeny::Deny;
        }
    }

    let funding_keypair: SECP256K1Keypair;
    match subnet_creation_decision {
        ApproveDeny::Approve => {
            println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

            println!("\n\nPlease select the Address that will Fund the Create Subnet Transaction. You will need to send some AVAX to this address,
The amount of AVAX required will be made clear later in the process.\n");
        
            println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));
            funding_keypair = Keys::generate_keypair(seed_phrase.to_string(), &select_single_account(None, seed_phrase));
        },
        ApproveDeny::Deny => {
            return Ok(ApproveDeny::Deny)
        },
    }


    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    println!("\n\nSubnets can have 'Control Keys', these are keypairs that the signatures of which is required in order for validators
to validate the subnet.\n");

    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    let multi_selected = &[
        "Yes",
        "No",
    ];

    let selection = Select::with_theme(&theme)
    .with_prompt("Do you want your Subnet to Have Control Keys?")
    .default(0)
    .items(&multi_selected[..])
    .interact()
    .unwrap();

    match selection {
        0 => {
            
        }
        1 => {

        }
        _ => {
            return Ok(ApproveDeny::Deny)
        }
    }

    todo!();
}


fn select_single_account(account_page: Option<u32>, seed_phrase: &String) -> String {

    let theme = XavaxTheme::default();

    let mut selection: Vec<String> = Vec::new();
    let mut current_account_page: u32 = 0;
    match account_page {
        Some(page) => {
            current_account_page = page;
            let mut i: u32 = page;
            while i < page + 5 {
                let path: String = DerivationPath::generate_path(Some(9000), i, 0, 0).get_path();
                let kp = Keys::generate_keypair(seed_phrase.to_string(), &path);
                let address: String = encode_bech32_address("fuji", kp.address.address_bytes);
                selection.push(format!("Account: {} | {} - {}", i, address, path));
                i += 1;
            }
            selection.push("Previous Accounts".to_string())
        }
        None => {
        }
    }
    let mut i: u32 = 0;
    while i <  5 {
        let path: String = DerivationPath::generate_path(Some(9000), i, 0, 0).get_path();
        let kp = Keys::generate_keypair(seed_phrase.to_string(), &path);
        let address: String = encode_bech32_address("fuji", kp.address.address_bytes);
        selection.push(format!("Account: {} | {} - {}", i, address, path));
        i += 1;
    }
    selection.push("Create More Accounts".to_string());

    let select_account: usize = Select::with_theme(&theme)
    .with_prompt("Select Address")
    .items(&selection)
    .default(0)
    .interact().expect("Select Error!");
    
    if selection[select_account] == "Create More Accounts" {
        current_account_page += 5;
        return select_single_account(Some(current_account_page), seed_phrase);
    } else if selection[select_account] == "Previous Accounts" {
        current_account_page -= 5;
        if current_account_page > 0 { return select_single_account(Some(current_account_page), seed_phrase); }
        else { return select_single_account(None, seed_phrase); }
    } else {
        return DerivationPath::generate_path(Some(9000), select_account as u32, 0, 0).get_path();
    }
}

fn select_multiple_accounts(seed_phrase: &String) {
    let theme = XavaxTheme::default();

    let mut selection: Vec<String> = Vec::new();

    let mut i: u32 = 0;
    while i <  5 {
        let path: String = DerivationPath::generate_path(Some(9000), i, 0, 0).get_path();
        let kp = Keys::generate_keypair(seed_phrase.to_string(), &path);
        let address: String = encode_bech32_address("fuji", kp.address.address_bytes);
        selection.push(format!("Account: {} | {} - {}", i, address, path));
        i += 1;
    }

    let select_account: Vec<usize> = MultiSelect::with_theme(&theme)
    .with_prompt("Select Address")
    .items(&selection)
    .defaults(&[true])
    .interact().expect("Select Error!");
}