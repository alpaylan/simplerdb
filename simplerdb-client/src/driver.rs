use crate::{connection::{Connection, Properties}, sql_exception::SQLException};



pub(crate) enum Driver {
    ClientDriver,
    EmbeddedDriver,
    NetworkDriver,
}

impl Driver {
    pub(crate) fn connect(&self, url: &str, props: Option<Properties>) -> Result<Connection, SQLException> {
        unimplemented!()
    }
}