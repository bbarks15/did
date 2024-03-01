use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use anyhow::Result;

use crate::record::Record;

pub trait Storage {
    fn add(&self, record: Record) -> Result<()>;
    fn list_all(&self) -> Result<Vec<Record>>;
}

pub struct Csv<'a> {
    pub path: &'a Path,
}

impl<'a> Storage for Csv<'a> {
    fn add(&self, record: Record) -> Result<()> {
        let needs_headers = !self.path.exists();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path)
            .unwrap();

        let mut writer = csv::WriterBuilder::new()
            .has_headers(needs_headers)
            .from_writer(file);

        writer.serialize(record)?;
        writer.flush()?;

        Ok(())
    }

    fn list_all(&self) -> Result<Vec<Record>> {
        let file = File::open(self.path)?;
        let mut reader = csv::Reader::from_reader(file);

        let result: Result<_, _> = reader.deserialize::<Record>().collect();

        Ok(result?)
    }
}
