use crate::BoxFuture;
use sqlx_core::transaction::*;
pub struct LibSqlTransactionManager;

impl TransactionManager for LibSqlTransactionManager {
    type Database = crate::LibSql;

    fn begin<'conn>(
        conn: &'conn mut <Self::Database as sqlx_core::database::Database>::Connection,
        statement: Option<std::borrow::Cow<'static, str>>,
    ) -> BoxFuture<'conn, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn commit(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn rollback(
        conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn start_rollback(conn: &mut <Self::Database as sqlx_core::database::Database>::Connection) {
        todo!()
    }

    fn get_transaction_depth(
        conn: &<Self::Database as sqlx_core::database::Database>::Connection,
    ) -> usize {
        todo!()
    }
}
