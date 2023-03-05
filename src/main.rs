use i3ipc::{reply::Node, I3Connection};
use std::env;

const STEP_COUNT: &str = "10 px or 10 ppt";
const ASSUMED_INNER_GAP: u8 = 20;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("invalid arguments ");
    }

    let move_to: Motions;
    let move_by: String;

    let dir_arg: &str = &args[1][..];
    match dir_arg {
        "left" => move_to = Motions::Left,
        "right" => move_to = Motions::Right,
        "up" => move_to = Motions::Up,
        "down" => move_to = Motions::Down,
        _ => panic!("invalid argument"),
    }

    let step_arg: &str;
    if args.len() > 2 {
        step_arg = &args[2][..];
        // todo!(" check if val is a valid number");
        move_by = format!("{step_arg} px or {step_arg} ppt");
    } else {
        move_by = String::from(STEP_COUNT);
    }
    (move_to, move_by)
}

fn main() {
    let (move_to, move_by) = get_args();
    let mut connection = I3Connection::connect().unwrap();
    let focused_node_rect = find_focused(&connection.get_tree().unwrap()).unwrap().rect;
    let w_spaces = connection.get_workspaces().unwrap();
    let workspace_rect = w_spaces.workspaces.get(0).unwrap().rect;

    let (mut upper_corner, mut bottom_corner, mut right_corner, mut left_corner) =
        (false, false, false, false);

    if focused_node_rect.0 - workspace_rect.0 < ASSUMED_INNER_GAP.into() {
        left_corner = true;
    }

    if focused_node_rect.1 - workspace_rect.1 < ASSUMED_INNER_GAP.into() {
        upper_corner = true;
    }

    if focused_node_rect.0 + focused_node_rect.2 == workspace_rect.2 {
        right_corner = true;
    }

    if focused_node_rect.1 + focused_node_rect.3 == workspace_rect.3 {
        bottom_corner = true;
    }

    let mut command: String = match move_to {
        Motions::Up => {
            if upper_corner && !bottom_corner {
                "resize shrink height ".into()
            } else {
                "resize grow height ".into()
            }
        }
        Motions::Down => {
            if upper_corner && !bottom_corner {
                "resize grow height ".into()
            } else {
                "resize shrink height ".into()
            }
        }
        Motions::Right => {
            if right_corner && !left_corner {
                "resize shrink width ".into()
            } else {
                "resize grow width ".into()
            }
        }
        Motions::Left => {
            if right_corner && !left_corner {
                "resize grow width ".into()
            } else {
                "resize shrink widht ".into()
            }
        }
    };
    command.push_str(&move_by);

    connection
        .run_command(&command)
        .expect("failed to send command");
}
