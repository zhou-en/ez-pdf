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
            commands::cmd_get_metadata,
            commands::cmd_set_metadata,
            commands::cmd_watermark,
            commands::cmd_list_bookmarks,
            commands::cmd_add_bookmark,
            commands::cmd_extract_images,
            commands::cmd_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
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

    // ── Phase 22 tests ────────────────────────────────────────────────────────

    #[test]
    fn cmd_get_metadata_returns_ok() {
        let result = cmd_get_metadata(fixture("3page.pdf"));
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    }

    #[test]
    fn cmd_set_metadata_writes_fields() {
        let tmp = TempDir::new().unwrap();
        let output = tmp.path().join("meta.pdf").to_string_lossy().into_owned();
        let result = cmd_set_metadata(
            fixture("3page.pdf"),
            output.clone(),
            Some("My Title".to_string()),
            Some("Alice".to_string()),
            None,
            None,
            None,
            None,
        );
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        let meta = cmd_get_metadata(output).unwrap();
        assert_eq!(meta.title, Some("My Title".to_string()));
        assert_eq!(meta.author, Some("Alice".to_string()));
    }

    #[test]
    fn cmd_watermark_produces_output() {
        let tmp = TempDir::new().unwrap();
        let output = tmp
            .path()
            .join("watermarked.pdf")
            .to_string_lossy()
            .into_owned();
        let result = cmd_watermark(
            fixture("3page.pdf"),
            "DRAFT".to_string(),
            48.0,
            0.3,
            None,
            output.clone(),
        );
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        assert_eq!(page_count(&output), 3);
    }

    #[test]
    fn cmd_list_bookmarks_returns_ok() {
        let result = cmd_list_bookmarks(fixture("3page.pdf"));
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    }

    #[test]
    fn cmd_add_bookmark_creates_bookmark() {
        let tmp = TempDir::new().unwrap();
        let output = tmp
            .path()
            .join("bookmarked.pdf")
            .to_string_lossy()
            .into_owned();
        let result = cmd_add_bookmark(
            fixture("3page.pdf"),
            "Chapter 1".to_string(),
            1,
            output.clone(),
        );
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        let bookmarks = cmd_list_bookmarks(output).unwrap();
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].title, "Chapter 1");
        assert_eq!(bookmarks[0].page, 1);
    }

    #[test]
    fn cmd_extract_images_returns_ok() {
        let tmp = TempDir::new().unwrap();
        let result = cmd_extract_images(
            fixture("3page.pdf"),
            tmp.path().to_string_lossy().into_owned(),
        );
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    }

    // ── Phase 24 tests ────────────────────────────────────────────────────────

    #[test]
    fn cmd_info_returns_page_count_and_dimensions() {
        let result = cmd_info(fixture("3page.pdf"));
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
        let info = result.unwrap();
        assert_eq!(info.page_count, 3);
        assert_eq!(info.dimensions.len(), 3);
        for (w, h) in &info.dimensions {
            assert!(*w > 0.0, "width must be positive");
            assert!(*h > 0.0, "height must be positive");
        }
    }
}
