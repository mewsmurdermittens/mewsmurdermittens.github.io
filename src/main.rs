use entrypoint::prelude::*;

#[entrypoint::entrypoint]
fn entrypoint(args: mewsmurdermittens::CLIArgs) -> entrypoint::anyhow::Result<()> {
    mewsmurdermittens::run(args.try_into()?)
}
