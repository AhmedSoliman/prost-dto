use prost_dto::IntoProst;

mod sub {
    #[repr(i32)]
    pub enum HttpMethod {
        Get,
        Post,
        Put,
    }
}

#[derive(IntoProst)]
#[prost(target = "sub::HttpMethod")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
}

fn main() {}
