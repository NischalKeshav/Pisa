use std::fs;

pub fn create_pisa_enviorment() -> Result<String, String>{
    // Create Pisa Enviorment for all important files and folders
    // TODO: use a enviormental .toml file for these variables
    let enviorment_directory = "./.pisa";
    fs::create_dir(enviorment_directory).ok();
   


    

    Ok("Success".to_string())
}


pub fn generate_activate_script() -> Result<String, String>{
    

    
    Ok("Success".to_string())

}
