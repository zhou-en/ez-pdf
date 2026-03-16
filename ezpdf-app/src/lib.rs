mod commands;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::cmd_merge,
            commands::cmd_split_range,
            commands::cmd_split_each,
            commands::cmd_remove,
            commands::cmd_rotate,
            commands::cmd_reorder,
            commands::cmd_page_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::commands::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("ezpdf-core/tests/fixtures")
    }

    fn fixture(name: &str) -> String {
        fixtures_dir().join(name).to_string_lossy().into_owned()
    }

    fn page_count(path: &str) -> u32 {
        let doc = lopdf::Document::load(path).unwrap();
        doc.get_pages().len() as u32
    }

    #[test]
    fn cmd_merge_combines_files() {
        let tmp = TempDir::new().unwrap();
        let output = tmp.path().join("merged.pdf").to_string_lossy().into_owned();
        let result = cmd_merge(
            vec![fixture("3page.pdf"), fixture("5page.pdf")],
            output.clone(),
        );
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 8);
    }

    #[test]
    fn cmd_merge_missing_input_returns_err() {
        let tmp = TempDir::new().unwrap();
        let output = tmp.path().join("out.pdf").to_string_lossy().into_owned();
        let result = cmd_merge(vec!["/nonexistent/file.pdf".to_string()], output);
        assert!(result.is_err());
    }

    #[test]
    fn cmd_split_range_produces_correct_pages() {
        let tmp = TempDir::new().unwrap();
        let output = tmp.path().join("split.pdf").to_string_lossy().into_owned();
        let result = cmd_split_range(fixture("5page.pdf"), "1-3".to_string(), output.clone());
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 3);
    }

    #[test]
    fn cmd_remove_removes_pages() {
        let tmp = TempDir::new().unwrap();
        let output = tmp
            .path()
            .join("removed.pdf")
            .to_string_lossy()
            .into_owned();
        let result = cmd_remove(fixture("3page.pdf"), "2".to_string(), output.clone());
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 2);
    }

    #[test]
    fn cmd_rotate_rotates_all_pages() {
        let tmp = TempDir::new().unwrap();
        let output = tmp
            .path()
            .join("rotated.pdf")
            .to_string_lossy()
            .into_owned();
        let result = cmd_rotate(fixture("3page.pdf"), 90, None, output.clone());
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 3);
    }

    #[test]
    fn cmd_reorder_changes_page_order() {
        let tmp = TempDir::new().unwrap();
        let output = tmp
            .path()
            .join("reordered.pdf")
            .to_string_lossy()
            .into_owned();
        let result = cmd_reorder(fixture("3page.pdf"), "3,1,2".to_string(), output.clone());
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 3);
    }
}
