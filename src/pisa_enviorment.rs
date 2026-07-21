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
