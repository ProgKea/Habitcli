use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "Habitcli", about = "A simple habit tracker written in rust")]
pub struct Arguments {
    // TODO add the commands seperately
    /// Command to run (add, remove, edit, checkin, list)
    #[structopt(required = true)]
    pub action: String,

    /// Name for the habit
    #[structopt(required = false, default_value = " ")]
    pub name: String,

    /// Count of the habit
    #[structopt(default_value = "0", required = false)]
    pub count: i32,

    /// Arguments to pass to the command
    pub arguments: Vec<String>,
}
