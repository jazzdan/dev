use watchexec::cli::Args;
use watchexec::cli::ArgsBuilder;
use watchexec::pathop::PathOp;
use watchexec::{error::Result, run::Handler};

pub struct DevHandler {}

impl Handler for DevHandler {
    fn on_manual(&self) -> Result<bool> {
        return Ok(true);
    }

    fn on_update(&self, ops: &[PathOp]) -> Result<bool> {
        println!("Saw updates: {:?}", ops);
        return Ok(true);
    }

    fn args(&self) -> Args {
        return ArgsBuilder::default()
            .cmd(vec!["echo hello world".into()])
            .paths(vec![".".into()])
            .build()
            .expect("mission failed");
    }
}
