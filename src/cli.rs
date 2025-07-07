use anyhow::Context;
use chrono::{Days, Local, Months, NaiveDate};
use clap::{Parser, Subcommand};

use crate::config::Config;

#[derive(Debug, Parser)]
/// Khanza Claim
///
/// An app to export khanza checkup to document for BPJS Kesehatan claiming purpose
pub struct KhanzaKlaimParser {
    #[command(subcommand)]
    pub command: KhanzaKlaimSubcommand,
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    #[command(flatten)]
    pub config: Config,
}

#[derive(Debug, Subcommand)]
pub enum KhanzaKlaimSubcommand {
    Raw(KhanzaKlaimRawCommand),
    Run(KhanzaKlaimRunSubcommand),
    Export(KhanzaKlaimExportSubcommand),
}

#[derive(Debug, Parser)]
/// run exporter in the background
pub struct KhanzaKlaimRunSubcommand {
    /// end date for the medical checkup documents to be exported
    #[arg(long,value_parser = date_value_parser,default_value="now")]
    pub to: chrono::NaiveDate,
    /// start date for the medical checkup documents to be exported
    #[arg(long,value_parser = date_value_parser,default_value="prev_month")]
    pub from: chrono::NaiveDate,

    #[arg(short)]
    pub force: bool,
}

#[derive(Debug, Parser)]
/// run exporter for single medical record and print it as raw html
pub struct KhanzaKlaimRawCommand {
    #[arg()]
    pub no_rawat: String,
}

#[derive(Debug, Parser)]
/// run exporter for single medical record
pub struct KhanzaKlaimExportSubcommand {
    #[arg()]
    pub no_rawat: String,
    #[arg(short)]
    pub force: bool,
}

fn date_value_parser(val: &str) -> anyhow::Result<NaiveDate> {
    let local = Local::now().naive_local().date();
    let error_message = "error when parsing date, valid value is : now, yesterday, prev_week, prev_month or date with format `YYYY-MM-DD`";

    Ok(match val {
        "now" => local,
        "yesterday" => local
            .checked_sub_days(Days::new(1))
            .context(error_message)?,
        "prev_week" => local
            .checked_sub_days(Days::new(7))
            .context(error_message)?,
        "prev_month" => local
            .checked_sub_months(Months::new(1))
            .context(error_message)?,
        other => NaiveDate::parse_from_str(other, "%Y-%m-%d").context(error_message)?,
    })
}
