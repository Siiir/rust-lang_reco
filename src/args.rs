#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = crate::exe_doc!())]
pub struct Args {
    /// Input file path – a path to file with text in one of supported languages.
    ///
    /// If no input file is given, app will read text from `stdin` instead.
    pub input_f_path: Option<String>,

    /// Measures this classifier's accuracy using testing data.
    ///
    /// This takes some time and prints additional message before reading user input.
    #[arg(short = 'A', long)]
    pub no_accuracy_measure: bool,
}
