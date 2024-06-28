use ::libc;
extern "C" {
    fn backupfile_internal(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
        _: bool,
    ) -> *mut libc::c_char;
}
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
#[no_mangle]
pub unsafe extern "C" fn backup_file_rename(
    mut dir_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut backup_type: backup_type,
) -> *mut libc::c_char {
    return backupfile_internal(dir_fd, file, backup_type, 1 as libc::c_int != 0);
}
