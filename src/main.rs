use std::{ io};
mod init;
mod pisa_enviorment;
fn main() -> io::Result<()>  {
    let args:Vec<String> = std::env::args().collect();
    if args.len()<=1{
        println!("Here's Pisa");
    } else if args[1]=="init"{
        init::create_pisa_enviorment()?;
    }









    Ok(())
}
