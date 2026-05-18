use swayipc::{Connection, Event, EventType, Node, WindowChange, NodeLayout, Floating};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connection 1: To send commands to Sway
    let mut cmd_conn = Connection::new()?;
    
    // Connection 2: To subscribe to the event stream
    let mut event_conn = Connection::new()?;
    let events = event_conn.subscribe([EventType::Window])?;

    println!("Sway Autotiler started. Super-optimized zero-allocation version.");

    for event in events {
        match event {
            Ok(Event::Window(w)) => {
                // We only care when a window gets focused
                if w.change == WindowChange::Focus {
                    let rect = w.container.rect;
                    
                    // Ultra-fast floating check using native Rust Enum matching (No string allocation)
                    let is_floating = w.container.node_type == swayipc::NodeType::FloatingCon 
                        || matches!(
                            w.container.floating,
                            Some(Floating::UserOn) | Some(Floating::AutoOn)
                        );
                    
                    if is_floating {
                        continue;
                    }

                    let mut is_tabbed_or_stacked = false;
                    let mut already_split_correctly = false;

                    // Determine split direction based on current window dimensions.
                    let target_cmd = if rect.width > rect.height { "split h" } else { "split v" };

                    // Fetch tree to check parent layout to prevent ruining tabbed/stacked layouts
                    // and to avoid infinite redundant nesting.
                    if let Ok(tree) = cmd_conn.get_tree() {
                        if let Some((_, Some(parent))) = get_focused_node_and_parent(&tree, None) {
                            
                            // Native Enum matching for layouts (Zero memory allocation)
                            is_tabbed_or_stacked = matches!(
                                parent.layout, 
                                NodeLayout::Tabbed | NodeLayout::Stacked
                            );

                            // Check if parent is ALREADY split in our target direction
                            already_split_correctly = match target_cmd {
                                "split h" => parent.layout == NodeLayout::SplitH,
                                "split v" => parent.layout == NodeLayout::SplitV,
                                _ => false,
                            };
                        }
                    }

                    // If we're inside a tabbed or stacked container, don't mess up the layout
                    if is_tabbed_or_stacked {
                        continue;
                    }

                    // Only send the split command if the parent layout isn't ALREADY 
                    // split in the target direction. This stops infinite container nesting!
                    if !already_split_correctly {
                        if let Err(e) = cmd_conn.run_command(target_cmd) {
                            eprintln!("Failed to execute command '{}': {}", target_cmd, e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                // Log the error and break the loop to prevent the infinite TTY flood when Sway exits.
                eprintln!("Sway IPC disconnected or encountered an error: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("Sway Autotiler exiting cleanly.");
    Ok(())
}

/// Recursively find the focused node and its parent in the Sway node tree.
fn get_focused_node_and_parent<'a>(
    node: &'a Node,
    parent: Option<&'a Node>,
) -> Option<(&'a Node, Option<&'a Node>)> {
    if node.focused {
        return Some((node, parent));
    }
    
    for child in &node.nodes {
        if let Some(res) = get_focused_node_and_parent(child, Some(node)) {
            return Some(res);
        }
    }
    
    for child in &node.floating_nodes {
        if let Some(res) = get_focused_node_and_parent(child, Some(node)) {
            return Some(res);
        }
    }
    
    None
}
