// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use anyhow::{Context, Result};

mod interpreter;

pub fn run() -> Result<i32> {
    let exe = std::env::current_exe().context("resolving current executable")?;
    interpreter::run_python(&exe, &std::env::args_os().skip(1).collect::<Vec<_>>())
}
fn main() {
    let exit_code = match run() {
        Ok(code) => code,
        Err(_e) => {
            1
        }
    };

    std::process::exit(exit_code);
}
