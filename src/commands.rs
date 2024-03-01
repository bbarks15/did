use crate::{record::Record, storage::Storage};
use anyhow::Result;
use chrono::Local;
use inquire::{required, DateSelect, Editor, Text};

pub fn add_record(storage: impl Storage) -> Result<()> {
    let today = Local::now().date_naive();
    let date = DateSelect::new("When did you do the thing?")
        .with_default(today)
        .with_max_date(today)
        .with_week_start(chrono::Weekday::Mon)
        .prompt()?;

    let title = Text::new("What did you do?")
        .with_validator(required!("A title is required"))
        .prompt()?;

    let description = Editor::new("Description:")
        .with_formatter(&|desc| match desc.len() {
            0 => "<skipped>".into(),
            1..=20 => desc.into(),
            _ => format!("{}...", &desc[..17]),
        })
        .prompt()?;

    let record = Record {
        title,
        description,
        date,
    };

    storage.add(record)
}

pub fn list_records(storage: impl Storage) -> Result<Vec<Record>> {
    storage.list_all()
}
