use clap::Parser;

fn main() -> anyhow::Result<()> {
    // App config.
    let app_args = lreco::Args::parse();
    let app_cfg = lreco::Cfg::new(app_args);

    // Classifier
    let l_classifier = lreco::create_classifier()?;
    // Accuracy measure
    lreco::opt_run_accuracy_measure(&app_cfg, &l_classifier)?;

    // User input
    lreco::anal::predict_user_provided_lang(l_classifier, app_cfg)?;

    // End
    Ok(())
}
