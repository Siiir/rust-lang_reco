use std::io::BufReader;

use anyhow::Context;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    // App config.
    let app_args = lreco::Args::parse();
    let app_cfg = lreco::Cfg::new(app_args);

    let l_classifier =
        lreco::create_classifier().context("Failed to create language recognizer.")?;
    // Accuracy measure
    if app_cfg.run_accuracy_measure {
        lreco::run_accuracy_measure(&l_classifier)?;
    }
    // The recognizer
    let l_reco = lreco::from_classifier(l_classifier)?;

    // User input.
    let mut buf_reader = BufReader::new(app_cfg.input_reader()?);
    // App output
    let pred_langs = l_reco(&mut buf_reader);
    dbg!(pred_langs);

    // End
    Ok(())
}
