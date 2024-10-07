use crate::{resultset::ResultSet, sql_exception::SQLException};


pub(crate) struct Statement {
    
}

impl Statement {
    pub(crate) fn execute_update(cmd: &str) -> Result<u64, SQLException> {
        unimplemented!()
    }

    pub(crate) fn execute_query(qry: &str) -> Result<ResultSet, SQLException> {
        unimplemented!()
    }
}