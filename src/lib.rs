mod arguments;
mod column;
mod connection;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_info;
mod value;

use sqlx_core::database::Database;

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// libSQL database driver.
#[derive(Debug)]
pub struct LibSql;

impl Database for LibSql {
    type Connection = crate::connection::LibSqlConnection;

    type TransactionManager = crate::transaction::LibSqlTransactionManager;

    type Row = crate::row::LibSqlRow;

    type QueryResult = crate::query_result::LibSqlQueryResult;

    type Column = crate::column::LibSqlColumn;

    type TypeInfo = crate::type_info::LibSqlTypeInfo;

    type Value = crate::value::LibSqlValue;

    type ValueRef<'r> = crate::value::LibSqlValueRef<'r>;

    type Arguments<'q> = crate::arguments::LibSqlArgument<'q>;

    type ArgumentBuffer<'q> = crate::arguments::LibSqlArgumentBuffer<'q>;

    type Statement<'q> = crate::statement::LibSqlStatement<'q>;

    const NAME: &'static str = "libSQL";

    const URL_SCHEMES: &'static [&'static str] = &["sqlite"];
}
