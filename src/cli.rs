use crate::args;
use crate::func;
use std::io::Error;

#[allow(unreachable_code, unused)]
pub fn handle_cmds(args: args::Arguments) -> anyhow::Result<()> {
    let cmd = &args.action;
    match &*cmd.to_lowercase() {
        "add" => func::add(args)?,
        "edit" => func::edit(args)?,
        "remove" => func::remove(args)?,
        "checkin" => func::checkin(args)?,
        "list" => func::list(args.name)?,
        "listall" => func::listall()?,
        _ => {
            return Err(anyhow::Error::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Command not found",
            )))
        }
    };
    Ok(())
}
