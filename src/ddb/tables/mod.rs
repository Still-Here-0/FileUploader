
mod board;
pub use board::Board;

mod column_type;
pub use column_type::ColumnType;

mod custom_sql_script;
pub use custom_sql_script::CustomSqlScript;

mod group;
pub use group::Group;

mod hist_group;
pub use hist_group::HistGroup;

mod hist_sheet;
pub use hist_sheet::HistSheet;

mod hist_uploader_permission;
pub use hist_uploader_permission::HistUploaderPermission;

mod hist_sheet_meta_data;
pub use hist_sheet_meta_data::HistSheetMetaData;

mod manager_permission;
pub use manager_permission::ManagerPermission;

mod profile_groups;
pub use profile_groups::ProfileGroups;

mod profile;
pub use profile::Profile;

mod sheet_meta_data;
pub use sheet_meta_data::SheetMetaData;

mod sheet_used_by_board;
pub use sheet_used_by_board::SheetUsedByBoard;

mod sheet;
use sheet::Sheet;

mod upload;
pub use upload::Upload;

mod uploader_permission;
pub use uploader_permission::UploaderPermission;

mod worker;
pub use worker::Worker;

#[cfg(test)]
mod tests {
    use crate::ddb::tables::hist_sheet::HistSheet;

    use super::*;
    use super::super::DBLoad;

    async fn check_table<T, const N: usize>() 
        where T: DBLoad<N>
    {
        dotenvy::dotenv().ok();
        let mut client = super::super::context::mssql_connect().await.unwrap();
    
        let tab_name = T::TAB;
        let columns = T::COLS;
    
        let mut stream = client
            .query(format!("SELECT TOP(1) * FROM uploader.[{tab_name}]"), &[],).await.unwrap();

        let slq_columns  = stream.columns().await.unwrap().unwrap().to_vec();

        for column in slq_columns {
            let column_name = column.name();
            let a = column.column_type();
            if !columns.contains(&column_name) {
                panic!("'{column_name}' not in '{tab_name}' definition")
            }
        }
    
        let a = T::from_stream(stream).await.unwrap();
    }

    #[tokio::test]
    async fn check_board() {
        type Table = Board;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_column_type() {
        type Table = ColumnType;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_custom_sql_script() {
        type Table = CustomSqlScript;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_group() {
        type Table = Group;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_hist_group() {
        type Table = HistGroup;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_hist_sheet() {
        type Table = HistSheet;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_hist_uploader_permission() {
        type Table = HistUploaderPermission;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_hist_sheet_meta_data() {
        type Table = HistSheetMetaData;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_manager_permission() {
        type Table = ManagerPermission;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_profile_groups() {
        type Table = ProfileGroups;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_profile() {
        type Table = Profile;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_sheet_meta_data() {
        type Table = SheetMetaData;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_sheet_used_by_board() {
        type Table = SheetUsedByBoard;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_sheet() {
        type Table = Sheet;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_upload() {
        type Table = Upload;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_uploader_permission() {
        type Table = UploaderPermission;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }

    #[tokio::test]
    async fn check_worker() {
        type Table = Worker;
        const LEN: usize = Table::COLS.len();
        check_table::<Table, LEN>().await
    }
}