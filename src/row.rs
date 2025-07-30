use sqlx_core::row::Row;

pub struct LibSqlRow {}

impl Row for LibSqlRow {
    type Database = crate::LibSql;

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        todo!()
    }

    fn try_get_raw<I>(
        &self,
        index: I,
    ) -> Result<<Self::Database as sqlx_core::database::Database>::ValueRef<'_>, sqlx_core::Error>
    where
        I: sqlx_core::column::ColumnIndex<Self>,
    {
        todo!()
    }
}
