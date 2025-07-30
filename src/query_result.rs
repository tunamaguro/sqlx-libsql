#[derive(Debug, Default)]
pub struct LibSqlQueryResult {
    pub rows_affected: usize,
}

impl Extend<Self> for LibSqlQueryResult {
    fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
        for result in iter {
            self.rows_affected += result.rows_affected;
        }
    }
}
