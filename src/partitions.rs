use crate::cmd::{sinfo, squeue};
use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Status {
    Pdp, // priority pending
    Pdd, // denpendency pending
    Pdo, // others pending
    Cg,  // complete
    R,   // running
}

impl Status {
    pub fn creat_status(source: &str, reason: &str) -> anyhow::Result<Self> {
        match source {
            "PD" => {
                if reason.contains("(Priority")
                    || reason.contains("(Resources")
                    || reason.contains("None")
                {
                    Ok(Status::Pdp)
                } else if reason.contains("(Dependency") {
                    Ok(Status::Pdd)
                } else {
                    Err(anyhow!("PD status cannot parse reason {}", reason))
                }
            }
            "R" => Ok(Self::R),
            "CG" => Ok(Self::Cg),
            _ => Err(anyhow!("parse status {} error", source)),
        }
    }
}

#[derive(Default, Debug)]
pub struct Partition {
    pub name: String,
    pub time_limit: String,
    pub status: HashMap<Status, usize>,
}

impl Partition {
    pub fn new(name: &str, time_limit: &str) -> Self {
        Self {
            name: name.to_string(),
            time_limit: time_limit.to_string(),
            status: HashMap::new(),
        }
    }

    pub fn complete(&self) -> usize {
        *self.status.get(&Status::Cg).unwrap_or(&0)
    }

    pub fn pendingo(&self) -> usize {
        *self.status.get(&Status::Pdo).unwrap_or(&0)
    }

    pub fn pendingd(&self) -> usize {
        *self.status.get(&Status::Pdd).unwrap_or(&0)
    }

    pub fn pendingp(&self) -> usize {
        *self.status.get(&Status::Pdp).unwrap_or(&0)
    }

    pub fn running(&self) -> usize {
        *self.status.get(&Status::R).unwrap_or(&0)
    }

    pub fn update(&mut self, status: Status, increment: Option<usize>) {
        self.status
            .entry(status)
            .and_modify(|counter| *counter += increment.unwrap_or(1))
            .or_insert(0);
    }
}

#[derive(Default, Debug)]
pub struct Partitions {
    pub data: HashMap<String, Partition>,
}

pub fn complete_partition_name<'a>(
    name: &'a str,
    current_partions: &'a [(String, String)],
) -> Option<&'a (String, String)> {
    for item in current_partions.iter() {
        if item.0 == name {
            return Some(item);
        }
    }

    current_partions.iter().find(|&item| item.0.contains(name))
}

#[test]
fn test_complete_partition_name() {
    let full_names = get_partition_names_time().unwrap();
    let mut partitions = Partitions::default();

    partitions.update_job_status();
    partitions.print_job_status();

    println!(
        "{}",
        complete_partition_name("genomics-", &full_names).unwrap().0
    );
}

pub fn get_partition_names_time() -> anyhow::Result<Vec<(String, String)>> {
    let output = sinfo(Some(["-s"].as_ref())).unwrap();
    let mut res = Vec::new();

    output.lines().skip(1).for_each(|item| {
        let mut words = item.split_whitespace();
        let name = words.next().unwrap().trim_matches('*').to_string();
        words.next();
        let time = words.next().unwrap().to_string();
        res.push((name, time));
    });

    Ok(res)
}

impl Partitions {
    pub fn new(data: HashMap<String, Partition>) -> Self {
        Self { data }
    }

    pub fn creat_partitions() -> anyhow::Result<Self> {
        // may be panic
        let output = sinfo(Some(&["-s"])).unwrap();
        let mut res = Self::default();

        for line in output.lines().skip(1) {
            let line_vec = line.split_whitespace().take(3).collect::<Vec<&str>>();
            if let [name, _avail, time_limit] = &line_vec[..] {
                let name = name.trim_matches('*').to_string();
                res.insert(name.as_str(), Partition::new(name.as_str(), time_limit));
            }
        }
        Ok(res)
    }

    pub fn partition_names(&self) -> Vec<&String> {
        let tmp: Vec<&String> = self.data.keys().collect();
        tmp
    }

    pub fn insert(&mut self, name: &str, partition: Partition) {
        self.data.insert(name.to_string(), partition);
    }

    pub fn update_job_status(&mut self) {
        let output = squeue(None).unwrap();

        for line in output.lines().skip(1) {
            let line_vec = line.split_whitespace().collect::<Vec<&str>>();

            if let [_jobid, partition, _job_name, _useid, status, _time, _nodes, reason] =
                &line_vec[..]
            {
                self.data
                    .entry(partition.to_string())
                    .and_modify(|item| {
                        item.update(Status::creat_status(status, reason).unwrap(), None)
                    })
                    .or_insert(Partition {
                        name: partition.to_string(),
                        ..Default::default()
                    });
            }
        }
    }

    pub fn print_job_status(&self) {
        use comfy_table::*;

        let partition_info = get_partition_names_time().unwrap();

        let mut table = Table::new();

        table.set_width(80).set_header(vec![
            Cell::new("Partition")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Running")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Completing")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Resources Pending")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Dependency Pending")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("Time-limit")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
        ]);

        for par in self.data.values() {
            let par_info = complete_partition_name(&par.name, &partition_info).unwrap();

            table.add_row(vec![
                Cell::new(&par_info.0).fg(Color::Green),
                Cell::new(par.running())
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
                Cell::new(par.complete())
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
                Cell::new(par.pendingp())
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
                Cell::new(par.pendingd())
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
                Cell::new(&par_info.1)
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
            ]);
        }

        println!("{table}");
    }

    pub fn print_partitions(&self) {
        use comfy_table::*;

        let mut table = Table::new();

        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Partition")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new("Time-limit")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
            ]);

        for par in self.data.values() {
            table.add_row(vec![
                Cell::new(&par.name).fg(Color::Green),
                Cell::new(&par.time_limit)
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Right),
            ]);
        }
        println!("{table}");
    }
}
