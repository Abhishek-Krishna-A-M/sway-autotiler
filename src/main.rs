use swayipc::{Connection, Event, EventType, WindowChange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connection 1: To send commands to Sway
    let mut cmd_conn = Connection::new()?;
    
    // Connection 2: To subscribe to the event stream
    let mut event_conn = Connection::new()?;
    let events = event_conn.subscribe([EventType::Window])?;

    println!("Sway Autotiler started. Listening for focus events...");

    for event in events {
        match event {
            Ok(Event::Window(w)) => {
                // We only care when a window gets focused
                if w.change == WindowChange::Focus {
                    let rect = w.container.rect;
                    
                    // Prevent trying to split floating windows
                    if w.container.node_type == swayipc::NodeType::FloatingCon {
                        continue;
                    }

                    // Compare width and height to determine split direction
                    if rect.width > rect.height {
                        // Split side-by-side
                        if let Err(e) = cmd_conn.run_command("split h") {
                            eprintln!("Failed to execute split h: {}", e);
                        }
                    } else {
                        // Split top-and-bottom
                        if let Err(e) = cmd_conn.run_command("split v") {
                            eprintln!("Failed to execute split v: {}", e);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error receiving event: {}", e),
            _ => unreachable!(),
        }
    }

    Ok(())
}
