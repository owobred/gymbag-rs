use crate::proto_defaults::OrProtoDefault;
use crate::{frame, proto_defaults};

#[derive(Clone)]
pub struct Game {
    pub states: Vec<GameState>,
}

impl Game {
    pub fn from_frames(frames: Vec<frame::Frame>) -> Self {
        let mut frames = frames.into_iter();
        let header: frame::HeaderFrame = frames
            .next()
            .unwrap()
            .try_into()
            .expect("Failed to read header");

        let mut states = Vec::with_capacity(frames.len());

        let last_state = None;
        for frame in frames {
            let state = GameState::from_frame(frame, last_state.clone(), header.clone());
            states.push(state);
        }

        Self { states }
    }

    pub fn get_x(&self) -> Vec<Vec<Vec<f64>>> {
        // this type is silly, its a list of groups of inputs, each of which have a list of values
        let xs = self
            .states
            .iter()
            .map(|state| state.get_x())
            .collect::<Vec<_>>();
        let state_groups = xs
            .windows(10)
            .map(|window| window.to_vec())
            .collect::<Vec<_>>();
        state_groups
    }

    pub fn get_y(&self) -> Vec<Vec<Vec<f64>>> {
        let ys = self
            .states
            .iter()
            .map(|state| state.get_y())
            .collect::<Vec<_>>();
        let state_groups = ys
            .windows(10)
            .map(|window| window.to_vec())
            .collect::<Vec<_>>();
        state_groups
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub data: frame::Frame,
    pub header: frame::HeaderFrame,
    pub last_velocity: frame::Vector2,
}

impl GameState {
    fn get_x(&self) -> Vec<f64> {
        // let dead_bodies = pad_vec(
        //     self.data
        //         .dead_bodies
        //         .clone()
        //         .expect("missing dead bodies message")
        //         .dead_bodies,
        //     3,
        //     proto_defaults::proto_default(),
        // );
        // let other_imposters = pad_vec(
        //     self.header.other_impostors.clone(),
        //     3,
        //     -1,
        // );
        // let nearby_doors = pad_vec(
        //     self.data
        //         .map
        //         .clone()
        //         .expect("missing map message")
        //         .nearby_doors,
        //     3,
        //     proto_defaults::proto_default(),
        // );
        // let nearby_vents = pad_vec(
        //     self.data
        //         .map
        //         .clone()
        //         .expect("missing map message")
        //         .nearby_vents,
        //     3,
        //     proto_defaults::proto_default(),
        // );
        // let other_players = pad_vec(
        //     self.data
        //         .other_players
        //         .clone()
        //         .expect("missing other players message")
        //         .last_seen_players,
        //     14,
        //     proto_defaults::proto_default(),
        // );
        
        let tasks: Vec<frame::TaskData> = pad_vec(
            self.data
                .tasks
                .clone()
                .expect("missing tasks message")
                .tasks,
            10,
            proto_defaults::proto_default(),
        );
        assert_eq!(tasks.len(), 10, "should have 10 tasks");

        let tasks_xs = tasks
            .into_iter()
            .map(|task: frame::TaskData| {
                let x = task.get_x();
                let (left, right) = x.split_at(3);
                [left.to_vec(), right.to_vec()]
            })
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(tasks_xs.len(), 20, "should have 20 values");

        // let first = tasks_xs[0].clone();
        // if first.len() != 3 {
        //     println!("?????");
        //     let new_xs = vec![first; tasks_xs.len()];
        //     tasks_xs = new_xs;
        // }
        // println!("len after ???? is {}", tasks_xs.clone().into_iter().flatten().collect::<Vec<_>>().len());

        let mut tasks_xs = tasks_xs
            .into_iter()
            .map(|task_data| -> Vec<f64> {
                let first = task_data[0] as i64;
                match first {
                    -1 => vec![f64::INFINITY, task_data[1], task_data[2], task_data[0]],
                    _ => pad_vec(task_data, 4, -1.0),
                } // i don't really understand the code in the python version of this
            })
            .collect::<Vec<_>>();

        debug_assert_eq!(
            tasks_xs
                .clone()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .len(),
            80,
            "should have 80 values"
        );

        tasks_xs.sort_by(|left, right| left[0].partial_cmp(&right[0]).expect("failed to compare"));
        let tasks_xs = tasks_xs
            .into_iter()
            .map(|task| vec![task[1], task[2]])
            .collect::<Vec<_>>();

        // let nearby_vents = self
        //     .data
        //     .map
        //     .clone()
        //     .expect("missing map message")
        //     .nearby_vents
        //     .into_iter()
        //     .map(|vent| frame::VentData {
        //         id: vent.id,
        //         position: vent.position,
        //         connecting_vents: pad_vec(vent.connecting_vents, 3, proto_defaults::proto_default()),
        //     })
        //     .collect::<Vec<_>>();

        let sabotage = self
            .data
            .tasks
            .clone()
            .expect("missing tasks message")
            .sabotage
            .or_proto_default();

        let mut output: Vec<f64> = Vec::new();

        output.push(
            match frame::RoleType::from_i32(self.header.role).expect("somehow was missing a role") {
                frame::RoleType::Impostor | frame::RoleType::Shapeshifter => 1.0,
                _ => 0.0,
            },
        );

        output.push(
            self.data
                .local_player
                .clone()
                .expect("missing local player message")
                .kill_cooldown as f64,
        );
        output.extend(self.last_velocity.get_x());
        output.extend(tasks_xs.into_iter().flatten());
        output.extend(sabotage.get_x());
        output.extend(
            self.data
                .local_player
                .clone()
                .expect("missing local player message")
                .raycast_obstacle_distances
                .clone()
                .into_iter()
                .map(|distance| distance as f64),
        );
        assert_eq!(output.len(), 58, "i think its meant to be 58 things?");
        output
    }

    fn get_y(&self) -> Vec<f64> {
        let local_player = self.data.local_player.clone().expect("missing local player message");

        let velocity_data = local_player
            .velocity
            .expect("missing velocity data")
            .get_x();
        
        let mut output = vec![0.0; 4];

        if velocity_data[0] > 0.5 {
            output[0] = 1.0;
        } else if velocity_data[0] < -0.5 {
            output[1] = 1.0;
        }

        if velocity_data[1] > 0.5 {
            output[2] = 1.0;
        } else if velocity_data[1] < -0.5 {
            output[3] = 1.0;
        }

        let mut result = vec![];
        result.extend(output);
        result.push(local_player.did_report as i64 as f64);
        result.push(local_player.did_vent as i64 as f64);
        result.push(local_player.did_kill as i64 as f64);
        result
    }
}

impl GameState {
    fn from_frame(
        frame: frame::Frame,
        last_state: Option<Self>,
        header: frame::HeaderFrame,
    ) -> Self {
        Self {
            data: frame,
            header,
            last_velocity: last_state
                .map(|state| {
                    state
                        .data
                        .local_player
                        .expect("missing local_player")
                        .velocity
                        .or_proto_default()
                })
                .or_proto_default(),
        }
    }
}

fn pad_vec<T: Clone + std::fmt::Debug>(mut existing: Vec<T>, length: usize, value: T) -> Vec<T> {
    assert!(
        length >= existing.len(),
        "vec was longer than length, and shouldn't {existing:?}"
    );
    existing.resize(length, value);
    existing
}

trait GetX: Clone {
    fn get_x(&self) -> Vec<f64>;
}

impl<T: proto_defaults::ProtoDefault + Clone + GetX> GetX for Option<T> {
    fn get_x(&self) -> Vec<f64> {
        self.clone().or_proto_default().get_x()
    }
}

impl GetX for frame::Vector2 {
    fn get_x(&self) -> Vec<f64> {
        vec![self.x as f64, self.y as f64]
    }
}

impl GetX for frame::TaskData {
    fn get_x(&self) -> Vec<f64> {
        pad_vec(
            self.consoles_of_interest.clone(),
            2,
            proto_defaults::proto_default(),
        )
        .into_iter()
        .take(2)
        .map(|console| console.get_x())
        .flatten()
        .collect::<Vec<_>>()
    }
}

impl GetX for frame::PositionData {
    fn get_x(&self) -> Vec<f64> {
        let mut output: Vec<f64> = Vec::new();
        output.push(self.total_distance as f64);
        output.extend(self.next_node_offset.get_x());

        output
    }
}
