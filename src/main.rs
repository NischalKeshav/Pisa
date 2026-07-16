use std::{ io};
mod init;
fn main() -> io::Result<()>  {
    init::create_pisa_enviorment()?;

    Ok(())
}
