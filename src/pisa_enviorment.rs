use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;


pub fn chmod_exec(relative_file_path: String) -> io::Result<()> {
    
    let mut perms = match fs::metadata(relative_file_path.clone()){
        Err(e) => panic!("Unable to get file metadata {:?}", e),
        Ok(f) => f.permissions(),
    };
    perms.set_mode(perms.mode() | 0o111);
    fs::set_permissions(relative_file_path, perms)?;
 
    Ok(())
}
const ACTIVATE: &str = include_str!("../assets/activate");
//const CONFIG: &str = include_str!("../assets/config.toml");
//const GITIGNORE: &str = include_str!("../assets/gitignore");

pub fn get_template(name: &str) -> Result<&'static str,()> {
    match name {
        "activate" => Ok(ACTIVATE),
        //"config.toml" => Ok(CONFIG),
        //"gitignore" => Ok(GITIGNORE),
        _ => Err(()),
    }
}
