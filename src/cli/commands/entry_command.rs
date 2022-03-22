use super::create_account::{ create_account, ask_for_account_creation };
use super::create_subnet::{create_subnet};
use crate::cli::custom_theme::XavaxTheme;
use console::Style;
use dialoguer::{ Select, Input };


pub enum ApproveDeny {
    Approve,
    Deny,
}

pub enum StartSelection {
    CreateSubnet,
    AddVmOrChainToSubnet
}

/** ## init_cli()
 `init_cli()` is the entry function to the command loop for the snvm tool.
*/
pub fn init_cli() {    
    let pink = Style::new().color256(197);
    let mut seed: String = "".to_string();

    if seed == "" {
        println!("Before you can use the tool, you need to create an account! \n",);
        match ask_for_account_creation().expect("error") {
            ApproveDeny::Approve => {
                match create_account() {
                    //If the user did create an account, we forward the user back to the start_selection.
                    Some(val) => {
                        seed = val;
                    },
                    None => {
                        println!("Okay, {}", pink.apply_to("▲ Have a good time! ▲"));
                        return;
                    },
                }
            },
            ApproveDeny::Deny => {
                println!("Okay, {}", pink.apply_to("▲ Have a good time! ▲"));
                return;
            },
        }
    }
    
    let start = start_selection().expect("Error!");
    match start {
        StartSelection::CreateSubnet => {
            create_subnet(&seed);
        },
        StartSelection::AddVmOrChainToSubnet => {
            println!("{} has been selected!", pink.apply_to("Add Vm Or Chain To Subnet"));
        },
    }
}

/** ## start_selection()
 CLI Command which prompts the user what they want to do with the tool, either create a subnet or add a chain to an existing one.
 */
pub fn start_selection() -> Result<StartSelection, String> {

    let theme = XavaxTheme::default();
    let multi_selected = &[
        "Create Subnet",
        "Add Chain to existing Subnet",
    ];

    let selection = Select::with_theme(&theme)
    .with_prompt("What would you like to do?")
    .default(0)
    .items(&multi_selected[..])
    .interact()
    .unwrap();

    match selection {
        0 => {
            return Ok(StartSelection::CreateSubnet);
        }
        1 => {
            return Ok(StartSelection::AddVmOrChainToSubnet);
        }
        _ => {
            return Err("Start selection input failed! No valid input selected...".to_string())
        }
    }
}
