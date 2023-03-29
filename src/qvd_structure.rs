use core::fmt;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct QvdTableHeader {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "CreatorDoc")]
    pub creator_doc: String,
    #[serde(rename = "Fields")]
    pub fields: Fields,
    #[serde(rename = "NoOfRecords")]
    pub no_of_records: u32,
    #[serde(rename = "RecordByteSize")]
    pub record_byte_size: usize,
    #[serde(rename = "Offset")]
    pub offset: usize,
    #[serde(rename = "Length")]
    pub length: usize,
}

impl fmt::Display for QvdTableHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "QVD Metadata\n-\nNumber of records: {}\nNumber of columns: {}\nColumns: {}", 
            self.no_of_records,
            self.fields.headers.len(), 
            self.fields
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "$value", default)]
    pub headers: Vec<QvdFieldHeader>,
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comma_separated = String::new();

        let first = match self.headers.first() {
            Some(field) => field.field_name.to_string(),
            None => return write!(f, ""),
        };

        comma_separated.push_str(&first);

        self.headers.iter().skip(1).for_each(|field| {
            comma_separated.push_str(", ");
            comma_separated.push_str(&field.field_name.to_string());
        });

        write!(f, "{}", comma_separated)
    }
}

#[derive(Debug, Deserialize)]
pub struct QvdFieldHeader {
    #[serde(rename = "FieldName")]
    pub field_name: String,
    #[serde(rename = "Offset")]
    pub offset: usize,
    #[serde(rename = "Length")]
    pub length: usize,
    #[serde(rename = "BitOffset")]
    pub bit_offset: usize,
    #[serde(rename = "BitWidth")]
    pub bit_width: usize,
    #[serde(rename = "Bias")]
    pub bias: i32,
}
