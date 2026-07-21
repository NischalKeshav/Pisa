use std::fs;
use std::io;

use crate::pisa_enviorment;
pub fn create_pisa_enviorment() -> io::Result<()> {
    // Create Pisa Enviorment for all important files and folders
    // TODO: use a enviormental .toml file for these variables
    let enviorment_directory = "./.pisa";
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
    let activate_script:String = " export PS1_org=$PS1
    export PS1=(Pisa)$PS1 
        ".to_string();

    fs::write(relative_file_path, activate_script).unwrap();
    Ok(())

}

pub fn generate_deactivate_script(relative_file_path: &String) -> io::Result<()>{
    let activate_script:String = "export PS1=$PS1_org \n".to_string();

    fs::write(relative_file_path, activate_script).unwrap();
    Ok(())

}




