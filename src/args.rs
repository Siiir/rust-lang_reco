#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = crate::exe_doc!())]
pub struct Args {
    /// Input file path – a path to file with text in one of supported natural languages.
    pub input_f_path: Option<String>,

    /// Measures this classifier's accuracy using testing data.
    #[arg(short = 'a', long, default_value_t = true)]
    pub run_accuracy_measure: bool,
}
