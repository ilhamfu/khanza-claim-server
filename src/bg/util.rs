use std::io::Write as _;

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Datelike;
use fantoccini::error::NewSessionError;

use crate::{
    dto::GetRawatError,
    report::{render_report, TemplateContext},
    AppContext,
};

pub static PAGE_WIDTH: u32 = 17 * 48u32;
pub const WEBDRIVER_URL: &str = "http://localhost:4444";

pub static MONTH_NAME: [&str; 12] = [
    "JANUARI",
    "FEBRUARI",
    "MARET",
    "APRIL",
    "MEI",
    "JUNI",
    "JULI",
    "AGUSTUS",
    "SEPTEMBER",
    "OKTOBER",
    "NOVEMBER",
    "DESEMBER",
];
pub fn get_print_command(
    height: u64,
) -> webdriver::command::WebDriverCommand<webdriver::command::VoidWebDriverExtensionCommand> {
    webdriver::command::WebDriverCommand::Print(webdriver::command::PrintParameters {
        orientation: webdriver::command::PrintOrientation::Portrait,
        scale: 1f64,
        background: true,
        page: webdriver::command::PrintPage {
            height: (height as f64) * 2.54 / 96f64,
            width: 21.59f64,
        },
        margin: webdriver::command::PrintMargins {
            bottom: 0f64,
            top: 0f64,
            left: 0f64,
            right: 0f64,
        },
        page_ranges: vec![],
        shrink_to_fit: false,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum ExportProcessError {
    #[error("error when creating temp file")]
    TempFileNotCreated(#[source] std::io::Error),
    #[error("error when writing to temp file")]
    TempFileWriteError(#[source] std::io::Error),
    #[error("error when converting temp file path")]
    TempFilePath,
    #[error("error when converting values after getting page height")]
    ConvertPageHeight,
    #[error("error when converting values after printing document")]
    ConvertPrintedDocument,
    #[error("error when decoding printed document from base64")]
    DecodePrintedDocument(#[source] base64::DecodeError),
    #[error("error when creating folder for exported document")]
    CreateExportedFolder(#[source] std::io::Error),
    #[error("error when writing to pdf")]
    WriteToPdf(#[source] std::io::Error),
    #[error(transparent)]
    WebDriver(#[from] WebDriverError),
}

#[derive(Debug, thiserror::Error)]
#[error("error when running webdriver command: {kind}")]
pub struct WebDriverError {
    #[source]
    pub source: fantoccini::error::CmdError,
    pub kind: WebDriverErrorKind,
}

#[derive(Debug)]
pub enum WebDriverErrorKind {
    OpeningTempFile,
    SettingPageWidth,
    GettingPageHeight,
    Printing,
}

impl std::fmt::Display for WebDriverErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            WebDriverErrorKind::OpeningTempFile => "opening temp file",
            WebDriverErrorKind::SettingPageWidth => "setting page width",
            WebDriverErrorKind::GettingPageHeight => "getting page hegiht",
            WebDriverErrorKind::Printing => "printing document",
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportCheckupError {
    #[error("medical checkup is not exists")]
    MedicalCheckupNotExist,
    #[error(transparent)]
    GetRawat(#[from] GetRawatError),
    #[error(transparent)]
    ExportProcess(#[from] ExportProcessError),
}

pub async fn create_client() -> Result<fantoccini::Client, NewSessionError> {
    let mut retry: u32 = 5;
    loop {
        match fantoccini::ClientBuilder::native()
            .capabilities({
                let mut res = serde_json::Map::new();
                res.insert(
                    "goog:chromeOptions".to_owned(),
                    serde_json::json! ({
                        "args":[
                            "--headless","--disable-gpu","--no-sandbox"
                        ]
                    }),
                );

                res
            })
            .connect(WEBDRIVER_URL)
            .await
        {
            Ok(client) => return Ok(client),
            Err(err) if retry > 0 => {
                retry -= 1;
                let wait = retry.pow(5 - retry);
                tracing::warn!(%err,"error when trying to connect to web driver at {}, retrying in {}s",WEBDRIVER_URL,wait);
                tokio::time::sleep(std::time::Duration::from_secs(wait as u64)).await;
            }
            Err(err) => return Err(err),
        }
    }
}

pub async fn export_checkup(
    context: &AppContext,
    client: &fantoccini::Client,
    no_rawat: &str,
) -> Result<(), ExportCheckupError> {
    let Some(reg_perika) = crate::dto::get_rawat(&context.pool, no_rawat).await? else {
        return Err(ExportCheckupError::MedicalCheckupNotExist);
    };

    let template_context = TemplateContext {
        config: &context.config.app_config,
    };

    let html = render_report(&template_context, &reg_perika);

    let mut temp_file = tempfile::Builder::new()
        .prefix("checkup-")
        .suffix(".html")
        .rand_bytes(5)
        .tempfile_in("./temp")
        .map_err(ExportProcessError::TempFileNotCreated)?;

    temp_file
        .write_all(String::from(html).as_bytes())
        .map_err(ExportProcessError::TempFileWriteError)?;

    let temp_document_path = url::Url::from_file_path(temp_file.path())
        .map_err(|_| ExportProcessError::TempFilePath)?
        .to_string();

    client.goto(&temp_document_path).await.map_err(|err| {
        ExportProcessError::WebDriver(WebDriverError {
            source: err,
            kind: WebDriverErrorKind::OpeningTempFile,
        })
    })?;
    client
        .set_window_size(PAGE_WIDTH, 300)
        .await
        .map_err(|err| {
            ExportProcessError::WebDriver(WebDriverError {
                source: err,
                kind: WebDriverErrorKind::SettingPageWidth,
            })
        })?;
    let page_height = client
        .execute(
            "return document.documentElement.scrollHeight",
            Vec::default(),
        )
        .await
        .map_err(|err| {
            ExportProcessError::WebDriver(WebDriverError {
                source: err,
                kind: WebDriverErrorKind::GettingPageHeight,
            })
        })?
        .as_u64()
        .ok_or(ExportProcessError::ConvertPageHeight)?;

    let Some(res) = client
        .issue_cmd(get_print_command(page_height + 20))
        .await
        .map_err(|err| {
            ExportProcessError::WebDriver(WebDriverError {
                source: err,
                kind: WebDriverErrorKind::Printing,
            })
        })?
        .as_str()
        .map(ToOwned::to_owned)
    else {
        return Err(ExportProcessError::ConvertPrintedDocument.into());
    };

    let decoded = BASE64_STANDARD
        .decode(res)
        .map_err(ExportProcessError::DecodePrintedDocument)?;

    let year = reg_perika.reg_periksa.tgl_registrasi.year();
    let month_1 = reg_perika.reg_periksa.tgl_registrasi.month();
    let month = MONTH_NAME[reg_perika.reg_periksa.tgl_registrasi.month0() as usize];
    let typ = reg_perika.reg_periksa.status_lanjut;

    tokio::fs::create_dir_all(format!("./exported/{year}/{month_1:02}-{month}//{typ}/"))
        .await
        .map_err(ExportProcessError::CreateExportedFolder)?;
    tokio::fs::write(
        format!(
            "./exported/{year}/{month_1:02}-{month}/{typ}/{}-{}.pdf",
            reg_perika.reg_periksa.no_rm, reg_perika.sep.no_sep,
        ),
        &decoded,
    )
    .await
    .map_err(ExportProcessError::WriteToPdf)?;
    Ok(())
}
