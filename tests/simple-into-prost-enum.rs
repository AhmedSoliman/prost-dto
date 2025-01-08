use prost_dto::{FromProst, IntoProst};

mod sub {
    pub struct RunAt {
        pub run_at: Vec<String>,
    }
    pub struct Cron {
        pub pattern: String,
    }
    pub mod schedule {
        pub enum Schedule {
            Cron(super::Cron),
            RunAt(super::RunAt),
        }
    }
    pub struct Schedule {
        pub schedule: Option<schedule::Schedule>,
    }
}

#[derive(Debug, IntoProst, FromProst)]
#[prost(target = "sub::Cron")]
pub struct Cron {
    pub pattern: String,
}

#[derive(Debug, IntoProst, FromProst)]
#[prost(target = "sub::RunAt")]
pub struct RunAt {
    #[prost(name = "run_at")]
    pub timepoints: Vec<String>,
}

#[derive(Debug, IntoProst, FromProst)]
#[prost(target = "sub::Schedule", oneof = "schedule")]
pub enum Schedule {
    #[prost(name = "Cron")]
    Recurring(Cron),
    RunAt(RunAt),
}

fn main() {}
