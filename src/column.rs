use sqlx_core::column::Column;

#[derive(Debug)]
pub struct LibSqlColumn {}

impl Column for LibSqlColumn {
    type Database = crate::LibSql;

    fn ordinal(&self) -> usize {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn type_info(&self) -> &<Self::Database as sqlx_core::database::Database>::TypeInfo {
        todo!()
    }
}
