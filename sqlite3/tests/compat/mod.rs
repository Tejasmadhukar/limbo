#[allow(non_camel_case_types)]
use std::ptr;

#[repr(C)]
struct sqlite3 {
    _private: [u8; 0],
}

#[repr(C)]
struct sqlite3_stmt {
    _private: [u8; 0],
}

#[cfg_attr(not(feature = "sqlite3"), link(name = "limbo_sqlite3"))]
#[cfg_attr(feature = "sqlite3", link(name = "sqlite3"))]
extern "C" {
    fn sqlite3_libversion() -> *const libc::c_char;
    fn sqlite3_libversion_number() -> i32;
    fn sqlite3_close(db: *mut sqlite3) -> i32;
    fn sqlite3_open(filename: *const libc::c_char, db: *mut *mut sqlite3) -> i32;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        sql: *const libc::c_char,
        n_bytes: i32,
        stmt: *mut *mut sqlite3_stmt,
        tail: *mut *const libc::c_char,
    ) -> i32;
    fn sqlite3_finalize(stmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_wal_checkpoint(db: *mut sqlite3, db_name: *const libc::c_char) -> i32;
    fn sqlite3_wal_checkpoint_v2(
        db: *mut sqlite3,
        db_name: *const libc::c_char,
        mode: i32,
        log_size: *mut i32,
        checkpoint_count: *mut i32,
    ) -> i32;
}

const SQLITE_OK: i32 = 0;
const SQLITE_MISUSE: i32 = 21;
const SQLITE_CANTOPEN: i32 = 14;
const SQLITE_CHECKPOINT_PASSIVE: i32 = 0;
const SQLITE_CHECKPOINT_FULL: i32 = 1;
const SQLITE_CHECKPOINT_RESTART: i32 = 2;
const SQLITE_CHECKPOINT_TRUNCATE: i32 = 3;

#[test]
fn test_libversion() {
    unsafe {
        let version = sqlite3_libversion();
        assert!(!version.is_null());
    }
}

#[test]
fn test_libversion_number() {
    unsafe {
        let version_num = sqlite3_libversion_number();
        assert!(version_num >= 3042000);
    }
}

#[test]
fn test_open_not_found() {
    unsafe {
        let mut db = ptr::null_mut();
        assert_eq!(
            sqlite3_open(b"not-found/local.db\0".as_ptr() as *const i8, &mut db),
            SQLITE_CANTOPEN
        );
    }
}

#[test]
fn test_open_existing() {
    unsafe {
        let mut db = ptr::null_mut();
        assert_eq!(
            sqlite3_open(b"../testing/testing.db\0".as_ptr() as *const i8, &mut db),
            SQLITE_OK
        );
        assert_eq!(sqlite3_close(db), SQLITE_OK);
    }
}

#[test]
fn test_close() {
    unsafe {
        assert_eq!(sqlite3_close(ptr::null_mut()), SQLITE_OK);
    }
}

#[test]
#[ignore]
fn test_prepare_misuse() {
    unsafe {
        let mut db = ptr::null_mut();
        assert_eq!(
            sqlite3_open(b"../testing/testing.db\0".as_ptr() as *const i8, &mut db),
            SQLITE_OK
        );

        let mut stmt = ptr::null_mut();
        assert_eq!(
            sqlite3_prepare_v2(
                db,
                b"SELECT 1\0".as_ptr() as *const i8,
                -1,
                &mut stmt,
                ptr::null_mut()
            ),
            SQLITE_OK
        );

        assert_eq!(sqlite3_finalize(stmt), SQLITE_OK);
        assert_eq!(sqlite3_close(db), SQLITE_OK);
    }
}

#[test]
#[ignore]
fn test_wal_checkpoint() {
    unsafe {
        // Test with NULL db handle
        assert_eq!(
            sqlite3_wal_checkpoint(ptr::null_mut(), ptr::null()),
            SQLITE_MISUSE
        );

        // Test with valid db
        let mut db = ptr::null_mut();
        assert_eq!(
            sqlite3_open(b"../testing/testing.db\0".as_ptr() as *const i8, &mut db),
            SQLITE_OK
        );
        assert_eq!(sqlite3_wal_checkpoint(db, ptr::null()), SQLITE_OK);
        assert_eq!(sqlite3_close(db), SQLITE_OK);
    }
}

#[test]
#[ignore]
fn test_wal_checkpoint_v2() {
    unsafe {
        // Test with NULL db handle
        assert_eq!(
            sqlite3_wal_checkpoint_v2(
                ptr::null_mut(),
                ptr::null(),
                SQLITE_CHECKPOINT_PASSIVE,
                ptr::null_mut(),
                ptr::null_mut()
            ),
            SQLITE_MISUSE
        );

        // Test with valid db
        let mut db = ptr::null_mut();
        assert_eq!(
            sqlite3_open(b"../testing/testing.db\0".as_ptr() as *const i8, &mut db),
            SQLITE_OK
        );

        let mut log_size = 0;
        let mut checkpoint_count = 0;

        // Test different checkpoint modes
        assert_eq!(
            sqlite3_wal_checkpoint_v2(
                db,
                ptr::null(),
                SQLITE_CHECKPOINT_PASSIVE,
                &mut log_size,
                &mut checkpoint_count
            ),
            SQLITE_OK
        );

        assert_eq!(
            sqlite3_wal_checkpoint_v2(
                db,
                ptr::null(),
                SQLITE_CHECKPOINT_FULL,
                &mut log_size,
                &mut checkpoint_count
            ),
            SQLITE_OK
        );

        assert_eq!(
            sqlite3_wal_checkpoint_v2(
                db,
                ptr::null(),
                SQLITE_CHECKPOINT_RESTART,
                &mut log_size,
                &mut checkpoint_count
            ),
            SQLITE_OK
        );

        assert_eq!(
            sqlite3_wal_checkpoint_v2(
                db,
                ptr::null(),
                SQLITE_CHECKPOINT_TRUNCATE,
                &mut log_size,
                &mut checkpoint_count
            ),
            SQLITE_OK
        );

        assert_eq!(sqlite3_close(db), SQLITE_OK);
    }
}
