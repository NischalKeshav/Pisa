use std::{ io};
mod init;
mod pisa_enviorment;
fn main() -> io::Result<()>  {
    init::create_pisa_enviorment()?;

    Ok(())
}
