use std::path::PathBuf;

use clap::{ArgAction, Parser};
use mc_rs_extract::{modules::ExtractModule, Version};

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub(crate) struct Args {
    /// Whether to disable logging
    ///
    /// By default, logging is enabled.
    #[arg(short = 'q', long, default_value = "true", action = ArgAction::SetFalse)]
    pub(crate) verbose: bool,

    /// Refresh the [`VersionManifest`](mc_rs_extract::VersionManifest)
    ///
    /// This is needed to get information about versions that have been released
    /// since the last time the [`VersionManifest`](mc_rs_extract::VersionManifest) was downloaded.
    #[arg(short, help = REFRESH_HELP, long, long_help = REFRESH_LONG_HELP)]
    pub(crate) refresh: bool,

    /// The [`Version`] to extract information from
    ///
    /// If not specified, the latest release version, as determined by the
    /// [`VersionManifest`](mc_rs_extract::VersionManifest), will be used.
    #[arg(short, help = VERSION_HELP, long, long_help = VERSION_LONG_HELP)]
    pub(crate) version: Option<Version>,

    /// The list of [`Module`s](mc_rs_extract::modules::Module) to run
    ///
    /// If none are specified, all modules will be run.
    #[arg(short, help = MODULES_HELP, long = "module", long_help = MODULES_LONG_HELP)]
    pub(crate) modules: Vec<ExtractModule>,

    /// The output file
    ///
    /// If not specified, the output will be written to stdout.
    #[arg(short, help = OUTPUT_HELP, long, long_help = OUTPUT_LONG_HELP)]
    pub(crate) output: Option<PathBuf>,
}

const REFRESH_HELP: &str = "Refresh the VersionManifest before extracting information.";

const REFRESH_LONG_HELP: &str = r"Refresh the VersionManifest before extracting information.

This is needed to get information about versions that have been
released since the last time the manifest was downloaded.";

const VERSION_HELP: &str = "The version to extract information from.";

const VERSION_LONG_HELP: &str = r"The version to extract information from.

If not specified, the latest release version, as determined by the VersionManifest, will be used.";

const MODULES_HELP: &str = "The list of modules to run.";

const MODULES_LONG_HELP: &str = r"The list of modules to run.

If none are specified, all modules will be run.";

const OUTPUT_HELP: &str = "The output file.";

const OUTPUT_LONG_HELP: &str = r"The output file.

If not specified, the output will be written to stdout.";
