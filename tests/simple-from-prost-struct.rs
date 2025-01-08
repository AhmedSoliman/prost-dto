use prost_dto::FromProst;

mod sub {
    pub struct Output {
        pub name: String,
    }
}

#[derive(FromProst)]
#[prost(target = "sub::Output")]
pub struct Output {
    #[from_prost(map = "perform")]
    pub name: String,
}

fn perform(name: String) -> String {
    String::to_string(&name)
}

fn main() {}
