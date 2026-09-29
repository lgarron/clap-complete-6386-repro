use std::io::stdout;
use std::process::exit;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::generator::generate;
use clap_complete::{Generator, Shell};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(name = "repo")]
pub(crate) struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoCommand,
}

#[derive(Args, Debug)]
pub(crate) struct BoilerplateArgs {
    #[command(subcommand)]
    command: BoilerplateCommand,
}

#[derive(Debug, Subcommand)]
enum BoilerplateCommand {
    Biome(TemplateFileArgs),
    RustToolchain(TemplateFileArgs),
}

#[derive(Args, Debug)]
pub(crate) struct TemplateFileArgs {
    #[clap(long)]
    test_option: bool,
    #[command(subcommand)]
    pub(crate) command: TemplateFileCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum TemplateFileCommand {
    Add(TemplateFileCreateArgs),
    Edit,
    Reveal,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct TemplateFileCreateArgs {
    #[clap(long)]
    overwrite: bool,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct BlankArgs {}

#[derive(Debug, Subcommand)]
pub(crate) enum RepoCommand {
    /// Set up boilerplate for the repo.
    Boilerplate(BoilerplateArgs),
    /// Print completions for the given shell.
    Completions(CompletionsArgs),
}

#[derive(Args, Debug)]
pub(crate) struct CompletionsArgs {
    /// Print completions for the given shell.
    /// These can be loaded/stored permanently (e.g. when using Homebrew), but they can also be sourced directly, e.g.:
    ///
    ///  repo completions fish | source # fish
    ///  source <(repo completions zsh) # zsh
    #[clap(verbatim_doc_comment, id = "SHELL")]
    shell: Shell,
}

fn completions_for_shell(cmd: &mut clap::Command, generator: impl Generator) {
    generate(generator, cmd, "test_command", &mut stdout());
}

fn main() {
    let mut command = RepoArgs::command();

    let args = RepoArgs::parse();
    if let RepoCommand::Completions(completions_args) = args.command {
        completions_for_shell(&mut command, completions_args.shell);
        exit(0);
    };

    // Actual arg handling would go here.
}
