use i3ipc::{reply::Node, I3Connection};
use std::env;

const STEP_COUNT: &str = "10 px or 10 ppt";
const ASSUMED_MAX_INNER_GAP: u8 = 20;

#[derive(Debug)]
enum Motions {
    Left,
    Right,
    Up,
    Down,
}

fn find_focused(node: &Node) -> Option<&Node> {
    if node.focused {
        Some(node)
    } else {
        if let Some(&want) = node.focus.get(0) {
            let child = node.nodes.iter().find(|n| want == n.id).unwrap();
            find_focused(child)
        } else {
            None
        }
    }
}

fn get_args() -> (Motions, String) {
    let move_to: Motions;
    let move_by: String;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No arguments supplied ");
    }

    let dir_arg: &str = &args[1][..].to_lowercase();
    match dir_arg {
        "left" => move_to = Motions::Left,
        "right" => move_to = Motions::Right,
        "up" => move_to = Motions::Up,
        "down" => move_to = Motions::Down,
        _ => panic!("invalid argument"),
    }

    if args.len() > 2 {
        let step_arg = &args[2][..].parse::<i32>().expect("Invalid argument passed");
        move_by = format!("{step_arg} px or {step_arg} ppt");
    } else {
        move_by = String::from(STEP_COUNT);
    }
    (move_to, move_by)
}

fn main() {
    let (move_to, move_by) = get_args();
    let mut connection = I3Connection::connect().expect("Failed to create connection");
    let tree = &connection.get_tree().expect("Failed to get Node");
    let focused_node_rect = match find_focused(tree) {
        Some(node) => node,
        None => panic!("Failed to find focused node"),
    }
    .rect;
    let w_spaces = connection
        .get_workspaces()
        .expect("Failed to get current workspaces");
    let workspace_rect = w_spaces
        .workspaces
        .get(0)
        .expect("No workspaces found")
        .rect;

    let left_corner = focused_node_rect.0 - workspace_rect.0 < ASSUMED_MAX_INNER_GAP.into();
    let upper_corner = focused_node_rect.1 - workspace_rect.1 < ASSUMED_MAX_INNER_GAP.into();
    let right_corner = focused_node_rect.0 + focused_node_rect.2 == workspace_rect.2;
    let bottom_corner = focused_node_rect.1 + focused_node_rect.3 == workspace_rect.3;

    let command: &str = match move_to {
        Motions::Up => {
            if upper_corner && !bottom_corner {
                "shrink height"
            } else {
                "grow height"
            }
        }
        Motions::Down => {
            if upper_corner && !bottom_corner {
                "grow height"
            } else {
                "shrink height"
            }
        }
        Motions::Right => {
            if right_corner && !left_corner {
                "shrink width"
            } else {
                "grow width"
            }
        }
        Motions::Left => {
            if right_corner && !left_corner {
                "grow width"
            } else {
                "shrink width"
            }
        }
    };
    let command = format!("resize {} {}", command, move_by);

    connection
        .run_command(&command)
        .expect("failed to send command");
}
