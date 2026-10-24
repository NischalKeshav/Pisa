use std::fs;
use std::io;

use crate::pisa_enviorment;
pub fn create_pisa_enviorment() -> io::Result<()> {
    // Create Pisa Enviorment for all important files and folders
    let enviorment_directory = env!("PISA_BINARY_PATH");
    fs::create_dir(enviorment_directory).ok();
    let activate_script_path = format!("{}/activate", enviorment_directory);
    let deactivate_script_path = format!("{}/deactivate", enviorment_directory);

    // Activation script
    generate_activate_script(&activate_script_path)?;   
    pisa_enviorment::chmod_exec(activate_script_path)?; 
    // Deactivation script
    generate_deactivate_script(&deactivate_script_path)?;    
    pisa_enviorment::chmod_exec(deactivate_script_path)?;    
    


    Ok(())
}


pub fn generate_activate_script(relative_file_path: &String) -> io::Result<()>{
    let activate_script = r#"#!/usr/bin/env bash
# Source this file to change the current shell's prompt.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    echo "Use: source ${BASH_SOURCE[0]}" >&2
    exit 1
fi

deactivate() {
    if [[ -n "${_PISA_OLD_PS1+x}" ]]; then
        PS1="$_PISA_OLD_PS1"
        unset _PISA_OLD_PS1
    fi
    unset -f deactivate
}

if [[ -z "${_PISA_OLD_PS1+x}" ]]; then
    _PISA_OLD_PS1="${PS1-}"
    PS1="(Pisa) ${PS1-}"
fi
"#;

    fs::write(relative_file_path, activate_script)

}

pub fn generate_deactivate_script(relative_file_path: &String) -> io::Result<()>{
    let deactivate_script = r#"#!/usr/bin/env bash
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    echo "Use: source ${BASH_SOURCE[0]}" >&2
    exit 1
fi

if declare -F deactivate >/dev/null; then
    deactivate
fi
"#;

    fs::write(relative_file_path, deactivate_script)

}

