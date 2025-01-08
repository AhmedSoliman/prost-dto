use prost_dto::IntoProst;

mod sub {
    pub struct Recurring {
        pub cron: String,
        pub timezone: String,
        pub limit: u64,
        pub remaining: u64,
        pub data: Vec<String>,
    }
}

mod subsub {
    pub fn to_string(d: &i32) -> String {
        d.to_string()
    }
}

#[derive(IntoProst, Debug, Clone, PartialEq)]
#[prost(target = "sub::Recurring")]
pub struct Recurring {
    #[prost(required)]
    pub cron: Option<String>,
    pub timezone: String,
    pub limit: u64,
    // restricted but will still be converted.
    pub(crate) remaining: u64,
    #[prost(skip)]
    pub stuff: String,
    #[into_prost(map = "subsub::to_string", map_by_ref)]
    pub data: Vec<i32>,
    // non-public will not be included in the proto
    internal: String,
}

fn main() {}
