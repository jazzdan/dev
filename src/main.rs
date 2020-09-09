use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

fn main() -> Result<()> {
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    watcher.watch(".", RecursiveMode::Recursive)?;
    println!("Hello world");

    Ok(())
}
