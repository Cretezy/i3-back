#![feature(let_chains)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use serde_derive::{Deserialize, Serialize};

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
    /// focused window. It then saves this to the config file.
    ///
    /// The config file is stored at $XDG_CONFIG_HOME/i3-back/config.toml or $HOME/.config/i3-back/config.toml
    Start {
        #[clap(long, short, action)]
        /// Prints extra debugging information.
        debug: bool,
    },

    /// Command to switch focus to the previous window.
    ///
    /// When ran, reads from the config file and sends a command using i3's RPC to focus the
    /// previous window.
    Switch,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct I3BackConfig {
    last_focused_id: Option<i64>,
}

fn main() -> Result<()> {
    let mut cfg: I3BackConfig =
        confy::load("i3-back", "config").with_context(|| "Could not read i3-back config")?;

    let cli = I3BackCli::parse();

    match &cli.command {
        I3BackCommands::Start { debug } => {
            // Reset currently stored last focused ID.
            cfg.last_focused_id = None;
            confy::store("i3-back", "config", cfg.clone())
                .with_context(|| "Could not write i3-back config")?;

            // Setup listener for i3 events. We'll be listening for window events.
            // This event is triggered whenever the focus changes.
            let mut listener = I3EventListener::connect()
                .with_context(|| "Could not connect to i3 (event listener)")?;
            listener
                .subscribe(&[Subscription::Window])
                .with_context(|| "Could not subscribe to i3 events")?;

            // Setup the RPC connect to i3. We'll use this to get the current i3 tree,
            // to find the currently focused window's ID.
            let mut connection =
                I3Connection::connect().with_context(|| "Could not connect to i3 (RPC)")?;

            let tree = connection
                .get_tree()
                .with_context(|| "Could not get i3 tree")?;
            let mut last_focused_id = find_focused_id(tree);

            // Start listening for events. Should no longer exit from this point on.
            for event in listener.listen() {
                if let Err(err) = event {
                    eprintln!("Error in event listener: {}", err);
                    continue;
                }

                match connection.get_tree() {
                    Ok(tree) => {
                        let focused_id = find_focused_id(tree);

                        if let Some(focused_id) = focused_id {
                            if let Some(last_focused_id) = last_focused_id && focused_id ==last_focused_id {
                                // Ignore if focus hasn't changed
                                continue;
                            }

                            if let Some(last_focused_id) = last_focused_id {
                                if *debug {
                                    eprintln!("Saving new last focused ID: {}", last_focused_id);
                                }

                                // Save the new last focused ID
                                cfg.last_focused_id = Some(last_focused_id);

                                if let Err(err) = confy::store("i3-back", "config", cfg.clone()) {
                                    eprintln!("Could write i3-back config: {}", err);
                                }
                            }

                            last_focused_id = Some(focused_id);
                        }
                    }
                    Err(err) => {
                        eprintln!("Could get i3 tree: {}", err);
                    }
                }
            }
        }
        I3BackCommands::Switch => match cfg.last_focused_id {
            Some(last_focused_id) => {
                // Setup the RPC connect to i3. We'll use this to send a focus command.
                let mut connection =
                    I3Connection::connect().with_context(|| "Could not connect to i3 (RPC)")?;

                // Send command to focus the previous window
                connection
                    .run_command(format!("[con_id={}] focus", last_focused_id).as_str())
                    .with_context(|| {
                        format!("Could not focus i3 window with ID {last_focused_id}")
                    })?;
            }
            None => eprintln!("No last focused window."),
        },
    }

    Ok(())
}

/// Traverses i3 tree to find which node (including floating) is focused.
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
