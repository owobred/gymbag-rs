pub mod decode;
pub mod game;
pub mod proto_defaults;
pub mod py;

pub mod frame {
    include!(concat!(env!("OUT_DIR"), "/gymbag.proto.rs"));
    // include!(concat!(env!("OUT_DIR"), "/gymbag.proto.serde.rs"));

    impl TryFrom<Frame> for HeaderFrame {
        // todo: consider something more ergonimic than &'static str
        type Error = &'static str;

        fn try_from(frame: Frame) -> Result<Self, Self::Error> {
            if let Some(header) = frame.header {
                Ok(header)
            } else {
                Err("Frame is not a header frame")
            }
        }
    }
}
