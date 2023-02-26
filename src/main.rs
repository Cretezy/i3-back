#![feature(let_chains)]
use std::process;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Change the name of the mark to set.
    #[arg(short, long, default_value = "_back")]
    mark: String,

    /// Print extra debugging information.
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mark = args.mark.clone();

    ctrlc::set_handler(move || {
        // Clear up mark when exiting
        let unmark_result: Result<()> = (|| {
            let mut connection =
                I3Connection::connect().with_context(|| "Could not connect to i3 (IPC)")?;

            connection
                .run_command(&format!("unmark {mark}"))
                .with_context(|| format!("Could not unset i3 mark {mark}"))?;

            Ok(())
        })();

        if let Err(err) = unmark_result {
            eprintln!("Error while exiting: {:?}", err);
            process::exit(1);
        }

        process::exit(0);
    })
    .with_context(|| "Could not set exit handler")?;

    loop {
        if let Err(err) = run(&args) {
            eprintln!("Error: {:?}", err)
        }

        eprintln!("\nRestarting in 0.5s");
        thread::sleep(Duration::from_millis(500));
        eprintln!();
    }
}

fn run(args: &Args) -> Result<()> {
    let mark = args.mark.clone();
    let debug = args.debug;

    if debug {
        println!("Starting i3 event listener");
    }

    // Setup listener for i3 events. We'll be listening for window events.
    // This event is triggered whenever the focus changes.
    let mut listener =
        I3EventListener::connect().with_context(|| "Could not connect to i3 (event listener)")?;
    listener
        .subscribe(&[Subscription::Window])
        .with_context(|| "Could not subscribe to i3 events")?;

    // Setup the IPC connect to i3. We'll use this to get the current i3 tree,
    // to find the currently focused window's ID.
    let mut connection =
        I3Connection::connect().with_context(|| "Could not connect to i3 (IPC)")?;

    // Get the intial focused window
    let tree = connection
        .get_tree()
        .with_context(|| "Could not get i3's tree")?;
    let mut last_focused_id = find_focused_id(tree);

    // Start listening for i3 events
    for event in listener.listen() {
        event.with_context(|| "Could not receive i3 event. This is normal when restarting i3")?;

        let tree = connection
            .get_tree()
            .with_context(|| "Could not get i3's tree")?;
        let focused_id = find_focused_id(tree);

        if let Some(focused_id) = focused_id {
            if let Some(last_focused_id) = last_focused_id && focused_id == last_focused_id {
                // Ignore if focused window ID hasn't changed
                continue;
            }

            if let Some(last_focused_id) = last_focused_id {
                if debug {
                    println!("Saving window ID {last_focused_id} to {mark} mark. Current window ID is {focused_id}")
                }

                // Save the new last focused ID as mark
                connection
                    .run_command(&format!("[con_id={last_focused_id}] mark --add {mark}"))
                    .with_context(|| "Could not set i3 mark {name} to {last_focused_id}")?;
            }

            last_focused_id = Some(focused_id);
        }
    }

    // Unreachable
    Ok(())
}

/// Traverses i3 tree to find which node (including floating) is focused.
fn find_focused_id(node: Node) -> Option<i64> {
    let mut node = node;

    while !node.focused {
        let focused_id = match node.focus.into_iter().next() {
            Some(focused_id) => focused_id,
            None => return None,
        };

        node = match node
            .nodes
            .into_iter()
            .chain(node.floating_nodes)
            .find(|n| n.id == focused_id)
        {
            Some(focused_id) => focused_id,
            None => return None,
        };
    }

    Some(node.id)
}
