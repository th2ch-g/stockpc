use clap::{ Parser, Subcommand };

#[derive(Parser, Debug, Clone)]
#[clap(version, about)] //#[clap(author, version, about)]
pub struct MainArg {

    #[clap(subcommand)]
    pub mode: Mode,

}

#[derive(Subcommand, Debug, Clone)]
pub enum Mode {

    /// add ticker
    #[clap(display_order = 1)]
    Add(AddArg),

    /// show stock price stored in file
    #[clap(display_order = 1)]
    Show(ShowArg),

    /// check ticker price
    #[clap(display_order = 2)]
    Check(CheckArg),

    /// remove ticker
    #[clap(display_order = 3)]
    Rm(RmArg),

    /// reset all data
    #[clap(display_order = 4)]
    Reset(ResetArg),

}

#[derive(Debug, clap::Args, Clone)]
pub struct AddArg {

    #[clap(short, long, help = "name", display_order = 1)]
    pub name: String,

    #[clap(short, long, help = "ticker", display_order = 2)]
    pub ticker: String,

    #[clap(short, long, help = "amount", display_order = 3)]
    pub amount: i32,

    #[clap(short, long, help = "purchase", display_order = 4)]
    pub price: i32,

    #[clap(short = '@', long, value_name = "STR", default_value = "~/.stockpc.csv", help = "Specify PATH to save")]
    pub path: String,

}

#[derive(Debug, clap::Args, Clone)]
pub struct ShowArg {

    #[clap(long, value_name = "STR", default_value = "~/.stockpc.csv", help = "Specify PATH to save")]
    pub path: String,

}

#[derive(Debug, clap::Args, Clone)]
pub struct CheckArg {

    #[clap(long, value_name = "STR", default_value = "https://minkabu.jp/stock/", help = "Specify URL to search")]
    pub url: String,

    #[clap(long, value_name = "STR", default_value = "~/.stockpc.csv", help = "Specify PATH to save")]
    pub path: String,

}

#[derive(Debug, clap::Args, Clone)]
pub struct RmArg {

    #[clap(short, long, help = "name", display_order = 1)]
    pub name: String,

    #[clap(long, value_name = "STR", default_value = "~/.stockpc.csv", help = "Specify PATH to save")]
    pub path: String,

}

#[derive(Debug, clap::Args, Clone)]
pub struct ResetArg {

    #[clap(long, value_name = "STR", default_value = "~/.stockpc.csv", help = "Specify PATH to save")]
    pub path: String,

}


pub fn arg() -> MainArg {
    MainArg::parse()
}
