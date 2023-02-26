#![feature(let_chains)]
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dbus::blocking::Connection;
use dbus::MethodErr;
use dbus_crossroads::Crossroads;
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct I3BackCli {
    #[command(subcommand)]
    command: I3BackCommands,
}

#[derive(Subcommand)]
enum I3BackCommands {
    /// Start command for the daemon.
    ///
    /// When ran, it listens to i3's socket for window events (which are triggered when focus
    /// changed). It then gets the i3 window tree and traverses it to find the ID of the currently
    /// focused window. It also runs a D-Bus server to receive switch commands from the client.
    Start {
        #[clap(long, short, action)]
        /// Prints extra debugging information.
        debug: bool,
    },

    /// Command to switch focus to the previous window.
    ///
    /// When ran, sends D-Bus command to the daemon to switch focus to previously focused window.
    /// previous window.
    Switch,
}

const DBUS_SERVICE: &str = "com.cretezy.I3Back";
const DBUS_METHOD_SWITCH: &str = "Switch";

fn main() -> Result<()> {
    let cli = I3BackCli::parse();

    match cli.command {
        I3BackCommands::Start { debug } => {
            // Store which window ID to focus on next switch
            let to_focus = Arc::new(RwLock::<Option<i64>>::new(None));

            // Setup D-Bus connection
            let c = Connection::new_session().with_context(|| "Could not connect to D-Bus")?;
            c.request_name(DBUS_SERVICE, false, true, false)
                .with_context(|| format!("Could not request D-Bus name for {DBUS_SERVICE}"))?;

            // Setup D-Bus server then register service and switch method
            let mut cr = Crossroads::new();
            let switch_iface_token = cr.register(DBUS_SERVICE, |b| {
                let to_focus = to_focus.clone();
                b.method(
                    DBUS_METHOD_SWITCH,
                    (),
                    (),
                    move |_: &mut dbus_crossroads::Context, _: &mut (), _: ()| {
                        // Setup the RPC connect to i3. We'll use this to send a focus command.
                        let mut connection = I3Connection::connect().map_err(|err| {
                            <dbus::Error as Into<MethodErr>>::into(dbus::Error::new_failed(
                                format!("Could not connect to i3 (IPC): {}", err).as_str(),
                            ))
                        })?;

                        if let Ok(to_focus) = to_focus.read() && let Some(to_focus) = *to_focus {
                            // Send i3 command to focus the previous window
                            if debug{
                                eprintln!("Switching to window with ID {to_focus}");
                            }

                            connection
                                .run_command(format!("[con_id={}] focus", to_focus).as_str())
                                .map_err(|err| {
                                    <dbus::Error as Into<MethodErr>>::into(dbus::Error::new_failed(
                                            format!(
                                                "Could not focus window with ID {to_focus}: {}",
                                                err
                                                )
                                            .as_str(),
                                            ))
                                })?;
                        }

                        Ok(())
                    },
                );
            });
            cr.insert("/", &[switch_iface_token], ());

            // Start i3 event listener
            thread::spawn(move || {
                let to_focus = to_focus.clone();

                'server: loop {
                    // Setup listener for i3 events. We'll be listening for window events.
                    // This event is triggered whenever the focus changes.
                    let mut listener = match I3EventListener::connect() {
                        Ok(listener) => listener,
                        Err(err) => {
                            eprintln!(
                                "Could not connect to i3 (event listener): {err}. Restarting in 1s"
                            );

                            // Restart
                            thread::sleep(Duration::from_secs(1));
                            continue 'server;
                        }
                    };
                    if let Err(err) = listener.subscribe(&[Subscription::Window]) {
                        eprintln!("Could not subscribe to i3 events: {err}. Restarting in 1s");

                        // Restart
                        thread::sleep(Duration::from_secs(1));
                        continue 'server;
                    };

                    // Setup the IPC connect to i3. We'll use this to get the current i3 tree,
                    // to find the currently focused window's ID.
                    let mut connection = match I3Connection::connect() {
                        Ok(connection) => connection,
                        Err(err) => {
                            eprintln!("Could not connect to i3 (IPC): {err}. Restarting in 1s");

                            // Restart
                            thread::sleep(Duration::from_secs(1));
                            continue 'server;
                        }
                    };

                    // Get the intial focused window
                    let tree = match connection.get_tree() {
                        Ok(tree) => tree,
                        Err(err) => {
                            eprintln!("Could not get i3's tree: {err}. Restarting in 1s");

                            // Restart
                            thread::sleep(Duration::from_secs(1));
                            continue 'server;
                        }
                    };
                    let mut last_focused_id = find_focused_id(tree);

                    // Start listening for i3 events
                    for event in listener.listen() {
                        if let Err(err) = event {
                            eprintln!("Could not receive i3 event: {}. Restarting in 1s", err);

                            // Restart
                            thread::sleep(Duration::from_secs(1));
                            continue 'server;
                        }

                        match connection.get_tree() {
                            Ok(tree) => {
                                let focused_id = find_focused_id(tree);

                                if let Some(focused_id) = focused_id {
                                    // Ignore if focused window ID hasn't changed
                                    if let Some(last_focused_id) = last_focused_id && focused_id ==last_focused_id {
                                        continue;
                                    }

                                    if let Some(last_focused_id) = last_focused_id {
                                        // Save the new last focused ID
                                        if debug {
                                            eprintln!(
                                                "Saving new last focused window with ID {last_focused_id}. Current focused window ID is {focused_id}"
                                                );
                                        }

                                        let mut to_focus = to_focus.write().unwrap();
                                        *to_focus = Some(last_focused_id);
                                    }

                                    last_focused_id = Some(focused_id);
                                }
                            }
                            Err(err) => {
                                eprintln!("Could not get i3's tree: {err}. Restarting in 1s");

                                // Restart
                                thread::sleep(Duration::from_secs(1));
                                continue 'server;
                            }
                        }
                    }
                }
            });

            // Start D-Bus server
            if let Err(err) = cr.serve(&c) {
                eprintln!("Error serving dbus server: {}", err);
            }
        }
        I3BackCommands::Switch => {
            // Setup D-Bus connection and client proxy, then send switch command
            let conn = Connection::new_session().with_context(|| "Could not connect to D_BUS")?;
            let proxy = conn.with_proxy(DBUS_SERVICE, "/", Duration::from_millis(5000));

            proxy
                .method_call(DBUS_SERVICE, DBUS_METHOD_SWITCH, ())
                .with_context(|| "Could not call switch D-Bus method")?;
        }
    }

    Ok(())
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
