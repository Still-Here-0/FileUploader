
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
pub use sheet::Sheet;

mod upload;
pub use upload::Upload;

mod uploader_permission;
pub use uploader_permission::UploaderPermission;

mod worker;
pub use worker::Worker;

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use crate::ddb::tables::hist_sheet::HistSheet;

    use super::*;
    use super::super::DBLoad;

    async fn check_table<T>() 
        where T: DBLoad
    {
        dotenvy::dotenv().ok();
        let mut client = super::super::context::functions::mssql_client().await.unwrap();
    
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
    
        let a = stream.into_row_stream().next().await;
    }

    #[tokio::test]
    async fn check_board() {
        check_table::<Board>().await
    }

    #[tokio::test]
    async fn check_column_type() {
        check_table::<ColumnType>().await
    }

    #[tokio::test]
    async fn check_custom_sql_script() {
        check_table::<CustomSqlScript>().await
    }

    #[tokio::test]
    async fn check_group() {
        check_table::<Group>().await
    }

    #[tokio::test]
    async fn check_hist_group() {
        check_table::<HistGroup>().await
    }

    #[tokio::test]
    async fn check_hist_sheet() {
        check_table::<HistSheet>().await
    }

    #[tokio::test]
    async fn check_hist_uploader_permission() {
        check_table::<HistUploaderPermission>().await
    }

    #[tokio::test]
    async fn check_hist_sheet_meta_data() {
        check_table::<HistSheetMetaData>().await
    }

    #[tokio::test]
    async fn check_manager_permission() {
        check_table::<ManagerPermission>().await
    }

    #[tokio::test]
    async fn check_profile_groups() {
        check_table::<ProfileGroups>().await
    }

    #[tokio::test]
    async fn check_profile() {
        check_table::<Profile>().await
    }

    #[tokio::test]
    async fn check_sheet_meta_data() {
        check_table::<SheetMetaData>().await
    }

    #[tokio::test]
    async fn check_sheet_used_by_board() {
        check_table::<SheetUsedByBoard>().await
    }

    #[tokio::test]
    async fn check_sheet() {
        check_table::<Sheet>().await
    }

    #[tokio::test]
    async fn check_upload() {
        check_table::<Upload>().await
    }

    #[tokio::test]
    async fn check_uploader_permission() {
        check_table::<UploaderPermission>().await
    }

    #[tokio::test]
    async fn check_worker() {
        check_table::<Worker>().await
    }
}