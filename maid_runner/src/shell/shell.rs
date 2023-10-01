use crate::core::{core::*, manual::*, utils::*};

use crate::modules::{
    curl::curl::*,
    // botnet::botnet::*,
    lookup::lookup::*,
    // attack::attack::*,
    maid_av::maid_av::*,
    // post_attack::post_attack::*,
    scanner::scanner::*,
};

// command structure
// bin [option] [value] ... [option_N] [value_N]

pub fn init() -> u8 {
    let system_input = system_exec_shell(false);

    // println!("{:?}", system_input);
    // system input are very time bigger than one and can't be iven
    if system_input.len() < 2 {
        system_exec_manual("default");
        return 1;
    }

    let module_name = system_input[1].as_str();

    match module_name {
        "core" | "c" => {
            shell_core(system_input);
        }

        "maid_av" | "av" => {
            shell_maid_av(system_input);
        }

        "lookup" | "l" => {
            shell_lookup(system_input);
        }

        "scanner" | "s" => {
            shell_scanner(system_input);
        }

        "bcurl" => {
            shell_curl(system_input);
        }

        "help" | "h" => {
            shell_manual(system_input);
        }

        _ => {
            let mut cmd = "".to_string();
            for item in &system_input[1..] {
                cmd = format!("{} {}", cmd, item);
            }

            system_text(&format!("[RAW_COMMAND] :: Trying exec → {}", cmd), "green");

            println!("{}", cmd);
            system_command_exec(&cmd, true);
        }
    }

    return 1;
}
