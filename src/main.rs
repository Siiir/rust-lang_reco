use std::io::BufReader;

use anyhow::Context;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    // App config.
    let app_args = lreco::Args::parse();
    let app_cfg = lreco::Cfg::new(app_args);

    // Recognizer
    let l_reco = lreco::create().context("Failed to create language recognizer.")?;

    // Accuracy measure
    if app_cfg.run_accuracy_measure {
        todo!()
    }

    // User input.
    let mut buf_reader = BufReader::new(app_cfg.input_reader().unwrap());
    // App output
    let pred_langs = l_reco(&mut buf_reader);
    dbg!(pred_langs);

    // End
    Ok(())
}
