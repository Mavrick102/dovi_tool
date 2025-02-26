use clap::{Args, ValueHint};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[clap(
        name = "json",
        long,
        short = 'j',
        help = "Sets the generator config JSON file to use",
        conflicts_with = "xml",
        required_unless_present = "xml",
        value_hint = ValueHint::FilePath
    )]
    pub json_file: Option<PathBuf>,

    #[clap(
        long,
        short = 'o',
        help = "Generated RPU output file location",
        value_hint = ValueHint::FilePath
    )]
    pub rpu_out: Option<PathBuf>,

    #[clap(
        name = "hdr10plus-json",
        long,
        help = "HDR10+ JSON file to generate from",
        conflicts_with = "madvr-file",
        value_hint = ValueHint::FilePath,
    )]
    pub hdr10plus_json: Option<PathBuf>,

    #[clap(
        short = 'x',
        long,
        help = "XML metadata file to generate from",
        conflicts_with_all = &["json", "hdr10plus-json", "madvr-file"],
        required_unless_present = "json",
        value_hint = ValueHint::FilePath
    )]
    pub xml: Option<PathBuf>,

    #[clap(long, help = "Canvas width for L5 metadata generation")]
    pub canvas_width: Option<u16>,

    #[clap(long, help = "Canvas height for L5 metadata generation")]
    pub canvas_height: Option<u16>,

    #[clap(
        name = "madvr-file",
        long,
        help = "madVR measurement file to generate from",
        value_hint = ValueHint::FilePath
    )]
    pub madvr_file: Option<PathBuf>,

    #[clap(
        long,
        help = "madVR source: use custom per-frame target nits if available"
    )]
    pub use_custom_targets: bool,
}
