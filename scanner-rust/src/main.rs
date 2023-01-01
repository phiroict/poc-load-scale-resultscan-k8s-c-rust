#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
unsafe fn main_0() -> libc::c_int {
    let mut read: ssize_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut nok: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    printf(b"Starting read!\n\0" as *const u8 as *const libc::c_char);
    let mut source: *const libc::c_char = b"/home/phiro/Dropbox/Projects/kubernetes/CKAD/training/6/scratch/testresults.jtl\0"
        as *const u8 as *const libc::c_char;
    let mut file_handle: *mut FILE = fopen(
        source,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    loop {
        read = getline(&mut line, &mut len, file_handle);
        if !(read != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        count += 1;
        let mut timestamp: *mut libc::c_char = strtok(
            line,
            b",\0" as *const u8 as *const libc::c_char,
        );
        let mut _elapsed: *mut libc::c_char = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
        let mut _label: *mut libc::c_char = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
        let mut resCode: *mut libc::c_char = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
        let mut timestamp_int: libc::c_long = atol(timestamp)
            / 1000 as libc::c_int as libc::c_long;
        let mut time: time_t = timestamp_int;
        let mut ts: tm = *localtime(&mut time);
        let mut buf: [libc::c_char; 80] = [0; 80];
        strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
            b"%a %Y-%m-%d %H:%M:%S ep:%s %Z\0" as *const u8 as *const libc::c_char,
            &mut ts,
        );
        if strcmp(resCode, b"200\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ok += 1;
        } else {
            printf(
                b"Non 200 error: %s - timestamp: %s \n\0" as *const u8
                    as *const libc::c_char,
                resCode,
                buf.as_mut_ptr(),
            );
            nok += 1;
        }
        if count % 1000000 as libc::c_int == 0 as libc::c_int {
            printf(
                b"Value epoch - time: %s (%s) resCode: %s, remainder %s:: ok:%d, nok:%d, total: %d\n\0"
                    as *const u8 as *const libc::c_char,
                timestamp,
                buf.as_mut_ptr(),
                resCode,
                line,
                ok,
                nok,
                count,
            );
        }
    }
    printf(
        b"Ending read, ok: %d, nok: %d, total records: %d!\n\0" as *const u8
            as *const libc::c_char,
        ok,
        nok,
        count,
    );
    fclose(file_handle);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
