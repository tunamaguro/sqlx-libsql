use sqlx_core::statement::Statement;

pub struct LibSqlStatement<'q> {
    _phantom: std::marker::PhantomData<&'q ()>,
}

impl<'q> Statement<'q> for LibSqlStatement<'q> {
    type Database = crate::LibSql;

    fn to_owned(&self) -> <Self::Database as sqlx_core::database::Database>::Statement<'static> {
        todo!()
    }

    fn sql(&self) -> &str {
        todo!()
    }

    fn parameters(
        &self,
    ) -> Option<
        sqlx_core::Either<&[<Self::Database as sqlx_core::database::Database>::TypeInfo], usize>,
    > {
        todo!()
    }

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        todo!()
    }

    fn query(
        &self,
    ) -> sqlx_core::query::Query<
        '_,
        Self::Database,
        <Self::Database as sqlx_core::database::Database>::Arguments<'_>,
    > {
        todo!()
    }

    fn query_with<'s, A>(&'s self, arguments: A) -> sqlx_core::query::Query<'s, Self::Database, A>
    where
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_as<O>(
        &self,
    ) -> sqlx_core::query_as::QueryAs<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx_core::database::Database>::Arguments<'_>,
    >
    where
        O: for<'r> sqlx_core::from_row::FromRow<
                'r,
                <Self::Database as sqlx_core::database::Database>::Row,
            >,
    {
        todo!()
    }

    fn query_as_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx_core::query_as::QueryAs<'s, Self::Database, O, A>
    where
        O: for<'r> sqlx_core::from_row::FromRow<
                'r,
                <Self::Database as sqlx_core::database::Database>::Row,
            >,
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }

    fn query_scalar<O>(
        &self,
    ) -> sqlx_core::query_scalar::QueryScalar<
        '_,
        Self::Database,
        O,
        <Self::Database as sqlx_core::database::Database>::Arguments<'_>,
    >
    where
        (O,): for<'r> sqlx_core::from_row::FromRow<
                'r,
                <Self::Database as sqlx_core::database::Database>::Row,
            >,
    {
        todo!()
    }

    fn query_scalar_with<'s, O, A>(
        &'s self,
        arguments: A,
    ) -> sqlx_core::query_scalar::QueryScalar<'s, Self::Database, O, A>
    where
        (O,): for<'r> sqlx_core::from_row::FromRow<
                'r,
                <Self::Database as sqlx_core::database::Database>::Row,
            >,
        A: sqlx_core::arguments::IntoArguments<'s, Self::Database>,
    {
        todo!()
    }
}
