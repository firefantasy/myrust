use clap::{ApSettings, Clap};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "yihailin <tickTockit@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    file_name: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(Test),
}



#[cfg(test)]
mod tests {
    #[test]
    
    fn it_works() {
        
        let Opts: Opts = Opts::parse();
    }
}
