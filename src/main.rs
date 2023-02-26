#![feature(let_chains)]
use std::thread;
use std::time::Duration;

use anyhow::{Context, Error};
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

fn main() {
    loop {
        if let Err(err) = run() {
            eprintln!("Error: {:?}", err)
        }

        eprintln!("\nRestarting in 1s");
        thread::sleep(Duration::from_secs(1));
        eprintln!();
    }
}

fn run() -> Result<(), Error> {
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
                // Save the new last focused ID as mark
                connection
                    .run_command(format!("[con_id={}] mark --add _back", last_focused_id).as_str())
                    .with_context(|| "Could not set i3 mark _back")?;
            }

            last_focused_id = Some(focused_id);
        }
    }

    Ok(())
}

/// Traverses i3 tree to find which node (including floating) is focused.
///
/// Only one node _should_ be focused at a time. This will return the first one.
fn find_focused_id(tree: Node) -> Option<i64> {
    if tree.focused {
        return Some(tree.id);
    }

    for child in tree.nodes {
        if let Some(focused_id) = find_focused_id(child) {
            return Some(focused_id);
        }
    }
    for child in tree.floating_nodes {
        if let Some(focused_id) = find_focused_id(child) {
            return Some(focused_id);
        }
    }

    None
}
