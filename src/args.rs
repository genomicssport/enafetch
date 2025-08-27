use clap::Parser;
#[derive(Debug, Parser)]
#[command(
    name = "rust-ena",
    version = "2.0",
    about = "ena fastq fetch.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct EnaArgs {
    /// please provide the path to the folder to be downloaded
    pub project_arg: String,
}
