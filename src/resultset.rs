


pub struct Record {

}

impl Record {
    pub(crate) fn get_string(&self, s: &str) -> String {
        unimplemented!()
    }

    pub(crate) fn get_int(&self, s: &str) -> i64 {
        unimplemented!()
    }

    pub(crate) fn get_float(&self, s: &str) -> f32 {
        unimplemented!()
    }

    pub(crate) fn get_double(&self, s: &str) -> f64 {
        unimplemented!()
    }

    pub(crate) fn get_time(&self, s: &str) -> chrono::DateTime<chrono::Utc> {
        unimplemented!()
    }

    pub(crate) fn get_date(&self, s: &str) -> chrono::DateTime<chrono::Utc> {
        unimplemented!()
    }
}
pub(crate) struct ResultSet {
    records: Vec<Record>,
    metadata: ResultSetMetadata
}

impl Iterator for ResultSet {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub(crate) struct ResultSetMetadata {
    columns: Vec<Column>,
}

pub(crate) struct Column {
    name: String,
    display_size: u64,
    ctype: ColumnType
}

pub(crate) enum ColumnType {
    Integer,
    VarChar
}