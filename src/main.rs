pub mod arg;
pub mod csv_util;
pub mod web;

use crate::arg::{*};

fn main() {

    let cli: MainArg = arg::arg();

    match &cli.mode {
        Mode::Add(add_arg) => {
            csv_util::make_csv(&add_arg.name, &add_arg.ticker, add_arg.amount, add_arg.price, &add_arg.path);
            csv_util::show_csv(&add_arg.path);
        },

        Mode::Show(show_arg) => {
            csv_util::show_csv(&show_arg.path);
        },

        Mode::Check(check_arg) => {
            csv_util::check_csv(&check_arg.url, &check_arg.path);
            csv_util::show_csv(&check_arg.path);
        },

        Mode::Rm(rm_arg) => {
            csv_util::rm_csv(&rm_arg.name, &rm_arg.path);
            csv_util::show_csv(&rm_arg.path);
        },

        Mode::Reset(reset_arg) => {
            csv_util::reset_csv(&reset_arg.path);
        },
    }
}
