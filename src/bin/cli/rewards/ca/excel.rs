use crate::cli::rewards::ca::excel::Error::MissingWorksheet;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CouldNotReadExcel(#[from] calamine::Error),

    #[error(transparent)]
    Xls(#[from] calamine::XlsxError),

    #[error(transparent)]
    CouldNotDeserialize(#[from] calamine::DeError),

    #[error("Couldn't find workbook {0}")]
    CouldNotFindWorkbook(String),

    #[error("Workbook is missing worksheet {0}")]
    MissingWorksheets(HashSet<String>),
}

pub fn read_ca_aggregated_file(
    filepath: &Path,
    tabs_list: &HashSet<String>,
) -> Result<HashMap<String, Range<DataType>>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(filepath)?;
    let worksheets = workbook.worksheets();
    let worksheets_names: HashSet<String> = worksheets.iter().map(|x| x.0.clone()).collect();
    if !tabs_list.is_subset(&worksheets_names) {
        let missing_sheets: HashSet<String> = tabs_list.difference(&worksheets_names).collect();
        return Err(MissingWorksheet(missing_sheets));
    }
    Ok(worksheets.into_iter().collect())
}
