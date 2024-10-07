use crate::{connection::{Connection, Properties}, sql_exception::SQLException};



pub(crate) enum Driver {
    Client,
    Embedded,
    Network,
}

impl Driver {
    pub(crate) fn connect(&self, url: &str, props: Option<Properties>) -> Result<Connection, SQLException> {
        unimplemented!()
    }
}