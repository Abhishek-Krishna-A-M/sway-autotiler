use swayipc::{Connection, Event, EventType, Node, WindowChange, NodeLayout, Floating};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connection 1: To send commands to Sway
    let mut cmd_conn = Connection::new()?;
    
    // Connection 2: To subscribe to the event stream
    let mut event_conn = Connection::new()?;
    
    // FIX: Listen to Focus, Floating, Move, and New events to capture every part of a window's life cycle
    let events = event_conn.subscribe([
        EventType::Window
    ])?;

    println!("Sway Autotiler started. Targeted ID-engine active.");

    for event in events {
        match event {
            Ok(Event::Window(w)) => {
                if matches!(
                    w.change,
                    WindowChange::Focus | WindowChange::Floating | WindowChange::Move | WindowChange::New
                ) {
                    let target_id = w.container.id;
                    
                    // Always pull a fresh tree to get post-calculation arrangement metrics
                    if let Ok(tree) = cmd_conn.get_tree() {
                        // FIX: Look up the exact container by ID instead of assuming global focus
                        if let Some((node, Some(parent))) = find_node_by_id_and_parent(&tree, target_id, None) {
                            
                            // Ultra-fast floating check using native Rust Enum matching (Zero string allocation)
                            let is_floating = node.node_type == swayipc::NodeType::FloatingCon 
                                || matches!(
                                    node.floating,
                                    Some(Floating::UserOn) | Some(Floating::AutoOn)
                                );
                            
                            if is_floating {
                                continue;
                            }

                            // Native Enum matching for layouts (Zero memory allocation)
                            let is_tabbed_or_stacked = matches!(
                                parent.layout, 
                                NodeLayout::Tabbed | NodeLayout::Stacked
                            );

                            if is_tabbed_or_stacked {
                                continue;
                            }

                            let rect = node.rect;
                            let split_dir = if rect.width > rect.height { "h" } else { "v" };

                            // Check if parent is ALREADY split in our target direction
                            let already_split_correctly = match split_dir {
                                "h" => parent.layout == NodeLayout::SplitH,
                                "v" => parent.layout == NodeLayout::SplitV,
                                _ => false,
                            };

                            // FIX: Target the window explicitly via its container ID. 
                            // (Note: The temporary String allocation here is instantly freed via RAII)
                            if !already_split_correctly {
                                let cmd = format!("[con_id={}] split {}", node.id, split_dir);
                                if let Err(e) = cmd_conn.run_command(&cmd) {
                                    eprintln!("Failed to execute command '{}': {}", cmd, e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Sway IPC disconnected or encountered an error: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("Sway Autotiler exiting cleanly.");
    Ok(())
}

/// FIX: Recursively find a specific node by its ID along with its parent container.
fn find_node_by_id_and_parent<'a>(
    node: &'a Node,
    target_id: i64,
    parent: Option<&'a Node>,
) -> Option<(&'a Node, Option<&'a Node>)> {
    if node.id == target_id {
        return Some((node, parent));
    }
    
    for child in &node.nodes {
        if let Some(res) = find_node_by_id_and_parent(child, target_id, Some(node)) {
            return Some(res);
        }
    }
    
    for child in &node.floating_nodes {
        if let Some(res) = find_node_by_id_and_parent(child, target_id, Some(node)) {
            return Some(res);
        }
    }
    
    None
}
