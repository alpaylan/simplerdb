use crate::sql_exception::SQLException;


pub(crate) struct Connection {
    auto_commit: bool,
    transaction_isolation: TransactionIsolation
}

pub(crate) enum TransactionIsolation {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable
}

impl Connection {
    pub(crate) fn create_statement(&self) {
        unimplemented!()
    }
    pub(crate) fn close(self) -> Result<(), SQLException> {
        unimplemented!()
    }
}

pub(crate) struct Properties {

}