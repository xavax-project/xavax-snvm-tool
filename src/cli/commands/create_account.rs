use crate::cli::custom_theme::XavaxTheme;
use console::Style;
use dialoguer::{Select, Input};

use xavax_avalanche::crypto::{utils::validate_mnemonic, engine::elliptic_curves::MnemonicKeypair};

use super::entry_command::{ApproveDeny};


//Command which asks the user if they want to create an account or not.
pub fn ask_for_account_creation() -> Result<ApproveDeny, String> {
    let theme = XavaxTheme::default();
    let multi_selected = &[
        "Yes",
        "No",
    ];

    let selection = Select::with_theme(&theme)
    .with_prompt("Would you like to create a new account? (seed-phrase)")
    .default(0)
    .items(&multi_selected[..])
    .interact()
    .unwrap();

    match selection {
        0 => {
            return Ok(ApproveDeny::Approve);
        }
        1 => {
            return Ok(ApproveDeny::Deny);
        }
        _ => {
            return Err("Start selection input failed! No valid input selected...".to_string())
        }
    }
}



//Gets an input from the user (which is supposed to be a mnemonic phrase), and makes sure that its valid.
//Loops over itself if invalid with an error message. 
fn get_valid_mnemonic() -> String {

    let pink = Style::new().color256(197);
    let pink_bold = Style::new().color256(197).bold();
    let theme = XavaxTheme::default();

    let input : String = Input::new()
    .with_prompt("Enter Seed-Phrase")
    .interact_text().expect("Error!");
    
    match validate_mnemonic(&input) {
        true => {
            input
        },
        false => {
            println!("{} Please try again~", pink.apply_to("Invalid Seed-Phrase!"));
            get_valid_mnemonic()
        },
    }
} 

//Command which prompts the user to create an "account" the tool will use for subnet creation.
pub fn create_account() -> Option<String> {
    let pink = Style::new().color256(197);
    let pink_bold = Style::new().color256(197).bold();
    let theme = XavaxTheme::default();

    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    println!("{}\n\nA 24-word seed phrase will be generated for you (unless you import one), the keypairs that will be derived from this seed-phrase
will be used to create the subnets & also authorize adding custom Chains to that subnet.", pink_bold.apply_to("▲ IMPORTANT ▲"));

    println!("\nMake sure to write down this seed-phrase somewhere safe, and share it with no one! If you lose it, you lose access to all things generated with the seed
as well as funds that may be stored in keypairs derived from the seed!");
    println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

    let multi_selected = &[
        "Yes",
        "Import Existing Seed-phrase",
        "No",
    ];

    let selection = Select::with_theme(&theme)
    .with_prompt("Would you like to proceed?")
    .default(0)
    .items(&multi_selected[..])
    .interact()
    .unwrap();

    match selection {
        0 => {
            let seed = xavax_avalanche::crypto::avalanche::keychain::Keys::generate_mnemonic_phrase();
            println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));
            println!("Please Write Down: {}", pink.apply_to(seed.clone()));
            println!("\nThe keypairs used to create the subnets and authorize adding blockchains, will be derived from this
seed phrase. {}", pink.apply_to("Share it with no one."));
            println!("\n{}", pink.apply_to("________________________________________________________________________________ \n"));

            return Some(seed);
        }
        1 => {
            return Some(get_valid_mnemonic());
        }
        2 => {
            return None;
        }
        _ => {
            return None;
        }
    }
}