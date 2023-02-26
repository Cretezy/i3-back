#![feature(let_chains)]
use std::thread;
use std::time::Duration;

use i3ipc::reply::Node;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

fn main() {
    loop {
        // Setup listener for i3 events. We'll be listening for window events.
        // This event is triggered whenever the focus changes.
        let mut listener = match I3EventListener::connect() {
            Ok(listener) => listener,
            Err(err) => {
                eprintln!("Could not connect to i3 (event listener): {err}. Restarting in 1s");

                // Restart
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };
        if let Err(err) = listener.subscribe(&[Subscription::Window]) {
            eprintln!("Could not subscribe to i3 events: {err}. Restarting in 1s");

            // Restart
            thread::sleep(Duration::from_secs(1));
            continue;
        };

        // Setup the IPC connect to i3. We'll use this to get the current i3 tree,
        // to find the currently focused window's ID.
        let mut connection = match I3Connection::connect() {
            Ok(connection) => connection,
            Err(err) => {
                eprintln!("Could not connect to i3 (IPC): {err}. Restarting in 1s");

                // Restart
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        // Get the intial focused window
        let tree = match connection.get_tree() {
            Ok(tree) => tree,
            Err(err) => {
                eprintln!("Could not get i3's tree: {err}. Restarting in 1s");

                // Restart
                thread::sleep(Duration::from_secs(1));
                continue;
            }
        };
        let mut last_focused_id = find_focused_id(tree);

        // Start listening for i3 events
        for event in listener.listen() {
            if let Err(err) = event {
                eprintln!("Could not receive i3 event: {err}. Restarting in 1s");

                // Restart
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            match connection.get_tree() {
                Ok(tree) => {
                    let focused_id = find_focused_id(tree);

                    if let Some(focused_id) = focused_id {
                        if let Some(last_focused_id) = last_focused_id && focused_id == last_focused_id {
                            // Ignore if focused window ID hasn't changed
                            continue;
                        }

                        if let Some(last_focused_id) = last_focused_id {
                            // Save the new last focused ID as mark
                            if let Err(err) = connection.run_command(
                                format!("[con_id={}] mark --add _back", last_focused_id).as_str(),
                            ) {
                                eprintln!("Could not set i3 mark _back: {err}. Restarting in 1s");

                                // Restart
                                thread::sleep(Duration::from_secs(1));
                                continue;
                            }
                        }

                        last_focused_id = Some(focused_id);
                    }
                }
                Err(err) => {
                    eprintln!("Could not get i3's tree: {err}. Restarting in 1s");

                    // Restart
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            }
        }
    }
}

/// Traverses i3 tree to find which node (including floating) is focused.
///
/// Only one node _should_ be focused at a time.
fn find_focused_id(node: Node) -> Option<i64> {
    let mut node = node;
    while !node.focused {
        let fid = match node.focus.into_iter().next() {
            Some(fid) => fid,
            None => return None,
        };
        node = match node.nodes.into_iter().find(|n| n.id == fid) {
            Some(fid) => fid,
            None => return None,
        };
    }
    Some(node.id)
}
