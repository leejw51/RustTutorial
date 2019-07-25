#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "make-cookie")]
struct MakeCookie {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    #[structopt(name = "tree")]
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt,Debug)]
enum Command {
    #[structopt(name = "pound")]
    /// Pound acorns into flour for cookie dough.
    Pound { acorns: u32 },
    #[structopt(name = "sparkle")]
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[structopt(short = "m", parse(from_occurrences))]
        magicality: u64,
        #[structopt(short = "c")]
        color: String,
    },
    #[structopt(name = "finish")]
    Finish(Finish),
    #[structopt(name = "hello", about = "hello world!")]
    Hello,
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt,Debug)]
struct Finish {
    #[structopt(short = "t")]
    time: u32,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    finish_type: FinishType,
}

// subsubcommand!
#[derive(StructOpt,Debug)]
enum FinishType {
    #[structopt(name = "glaze")]
    Glaze { applications: u32 },
    #[structopt(name = "powder")]
    Powder { flavor: String, dips: u32 },
}
fn main() {
    let opt = MakeCookie::from_args();
    println!("opt={:?}", opt);

}