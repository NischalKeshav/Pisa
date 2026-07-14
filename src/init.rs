use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
pub fn create_pisa_enviorment() -> Result<String, io::Error>{
    // Create Pisa Enviorment for all important files and folders
    // TODO: use a enviormental .toml file for these variables
    let enviorment_directory = "./.pisa";
    fs::create_dir(enviorment_directory).ok();
    let activate_script_path = format!("{}/activate", enviorment_directory);


    // Activation script
    let _ = generate_activate_script(&activate_script_path);   
    let mut perms = match fs::metadata(activate_script_path.clone()){
        Err(e) => panic!("Unable to get file metadata {:?}", e),
        Ok(f) => f.permissions(),
    };
    perms.set_mode(perms.mode() | 0o111);

    
    fs::set_permissions(activate_script_path, perms)?;
    Ok("Success".to_string())
}


pub fn generate_activate_script(relative_file_path: &String) -> Result<String, String>{
    let activate_script:String = "export PS1=(Pisa)$PS1 \n".to_string();

    fs::write(relative_file_path, activate_script).unwrap();
    Ok("Success".to_string())

}
