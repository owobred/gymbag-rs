use crate::frame;

pub fn proto_default<D: ProtoDefault>() -> D {
    D::proto_default()
}

pub trait ProtoDefault {
    fn proto_default() -> Self;
}

pub trait OrProtoDefault<T: ProtoDefault> {
    fn or_proto_default(self) -> T;
}

impl<T: ProtoDefault> OrProtoDefault<T> for Option<T> {
    fn or_proto_default(self) -> T {
        match self {
            Some(value) => value,
            None => T::proto_default(),
        }
    }
}

impl<T: ProtoDefault> ProtoDefault for Option<T> {
    fn proto_default() -> Self {
        Some(T::proto_default())
    }
}

impl ProtoDefault for frame::Vector2 {
    fn proto_default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl ProtoDefault for frame::PositionData {
    fn proto_default() -> Self {
        Self {
            total_distance: -1.0,
            next_node_position: proto_default(),
            next_node_offset: None,
        }
    }
}

impl ProtoDefault for frame::TaskData {
    fn proto_default() -> Self {
        Self {
            id: -1,
            r#type: frame::TaskType::NoneTaskType as i32,
            consoles_of_interest: vec![proto_default(); 2],
        }
    }
}

impl ProtoDefault for frame::UsableData {
    fn proto_default() -> Self {
        Self {
            r#type: 0,
            direction: Some(frame::Vector2::proto_default()),
        }
    }
}

impl ProtoDefault for frame::DeadBodyData {
    fn proto_default() -> Self {
        Self {
            parent_id: -1,
            position: proto_default(),
            first_seen_time: 0.0,
            nearby_players: vec![],
        }
    }
}

impl ProtoDefault for frame::DoorData {
    fn proto_default() -> Self {
        Self { position: proto_default(), is_open: false }
    }
}

impl ProtoDefault for frame::vent_data::ConnectingVentData {
    fn proto_default() -> Self {
        Self {
            id: -1,
            position: proto_default(),
        }
    }
}

impl ProtoDefault for frame::VentData {
    fn proto_default() -> Self {
        Self {
            id: -1,
            position: proto_default(),
            connecting_vents: vec![frame::vent_data::ConnectingVentData::proto_default(); 3],
        }
    }
}

impl ProtoDefault for frame::OtherPlayerData {
    fn proto_default() -> Self {
        Self {
            id: -1,
            last_seen_position: proto_default(),
            last_seen_time: -1.0,
            times_saw_vent: 0,
            round_time_visible: 0.0,
            game_time_visible: 0.0,
            is_visible: false,
        }
    }
}