pub mod image;

pub mod proto {
    #![allow(clippy::pedantic)]
    tonic::include_proto!("resizer");


    impl TryFrom<String> for ResizeMethod {
        type Error = &'static str;


        fn try_from(input: String) -> Result<Self, Self::Error> {
            match input.to_lowercase().as_str() {
                "fill" => Ok(ResizeMethod::Fill),
                "fit" => Ok(ResizeMethod::Fit),
                "limit" => Ok(ResizeMethod::Limit),
                "pad" => Ok(ResizeMethod::Pad),
                _ => Err("invalid resize format")
            }
        }
    }
}
