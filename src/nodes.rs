pub mod node {
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub struct ParseStatusError;

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Status {
        Pendingp, // pending priority
        Pendingd, // pending dependency
        Running,  // running
    }

    pub const SETS: [&str; 3] = ["pendingp", "pendings", "running"];

    impl FromStr for Status {
        type Err = ParseStatusError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "pendingp" => Ok(Status::Pendingp),
                "pendingd" => Ok(Status::Pendingd),
                "running" => Ok(Status::Running),
                _ => Err(ParseStatusError),
            }
        }
    }

    impl Status {}

    pub struct Node {
        name: String,
        tasks: HashMap<Status, usize>,
    }

    impl Node {
        pub fn pendingp(&self) -> usize {
            *self.tasks.get(&Status::Pendingp).unwrap_or(&0)
        }

        pub fn pendingd(&self) -> usize {
            *self.tasks.get(&Status::Pendingd).unwrap_or(&0)
        }

        pub fn running(&self) -> usize {
            *self.tasks.get(&Status::Running).unwrap_or(&0)
        }

        pub fn update(&mut self, status: Status) {
            self.tasks
                .entry(status)
                .and_modify(|counter| *counter += 1)
                .or_insert(0);
        }
    }
}
