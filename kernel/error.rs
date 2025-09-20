// SPDX-License-Identifier: GPL-2.0

//! Errors and results.
//!
//! Refer to linux

use core::fmt;

/// Generic error code.
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Operation not permitted.
    Eperm = 1,
    /// No such file or directory.
    Enoent = 2,
    /// No such process.
    Esrch = 3,
    /// Interrupted system call.
    Eintr = 4,
    /// Input/output error.
    Eio = 5,
    /// No such device or address.
    Enxio = 6,
    /// Argument list too long.
    E2big = 7,
    /// Exec format error.
    Enoexec = 8,
    /// Bad file number.
    Ebadf = 9,
    /// No child processes.
    Echild = 10,
    /// Try again.
    Eagain = 11,
    /// Out of memory.
    Enomem = 12,
    /// Permission denied.
    Eacces = 13,
    /// Bad address.
    Efault = 14,
    /// Block device required.
    Enotblk = 15,
    /// Device or resource busy.
    Ebusy = 16,
    /// File exists.
    Eexist = 17,
    /// Cross-device link.
    Exdev = 18,
    /// No such device.
    Enodev = 19,
    /// Not a directory.
    Enotdir = 20,
    /// Is a directory.
    Eisdir = 21,
    /// Invalid argument.
    Einval = 22,
    /// File table overflow.
    Enfile = 23,
    /// Too many open files.
    Emfile = 24,
    /// Not a typewriter.
    Enotty = 25,
    /// Text file busy.
    Etxtbsy = 26,
    /// File too large.
    Efbig = 27,
    /// No space left on device.
    Enospc = 28,
    /// Illegal seek.
    Espipe = 29,
    /// Read-only file system.
    Erofs = 30,
    /// Too many links.
    Emlink = 31,
    /// Broken pipe.
    Epipe = 32,
    /// Math argument out of domain of func.
    Edom = 33,
    /// Math result not representable.
    Erange = 34,
    /// Resource deadlock would occur.
    Edeadlk = 35,
    /// File name too long.
    Enametoolong = 36,
    /// No record locks available.
    Enolck = 37,
    /// Function not implemented.
    Enosys = 38,
    /// Directory not empty.
    Enotempty = 39,
    /// Too many symbolic links encountered.
    Eloop = 40,
    /// No message of desired type.
    Enomsg = 42,
    /// Identifier removed.
    Eidrm = 43,
    /// Channel number out of range.
    Echrng = 44,
    /// Level 2 not synchronized.
    El2nsync = 45,
    /// Level 3 halted.
    El3hlt = 46,
    /// Level 3 reset.
    El3rst = 47,
    /// Link number out of range.
    Elnrng = 48,
    /// Protocol driver not attached.
    Eunatch = 49,
    /// No CSI structure available.
    Enocsi = 50,
    /// Level 2 halted.
    El2hlt = 51,
    /// Invalid exchange.
    Ebaad = 52,
    /// Invalid request descriptor.
    Ebadr = 53,
    /// Exchange full.
    Exfull = 54,
    /// No anode.
    Enoano = 55,
    /// Invalid request code.
    Ebadrqc = 56,
    /// Invalid slot.
    Ebadslt = 57,
    /// Resource deadlock would occur.
    //Edeadlk = 58,
    /// Bad font file format.
    Ebf = 59,
    /// Device not a stream.
    Enostream = 60,
    /// No data available.
    Enodata = 61,
    /// Timer expired.
    Etime = 62,
    /// Out of streams resources.
    Enosr = 63,
    /// Machine is not on the network.
    Enonet = 64,
    /// Package not installed.
    Enopkg = 65,
    /// Object is remote.
    Eremote = 66,
    /// Link has been severed.
    Enolink = 67,
    /// Advertise error.
    Eadv = 68,
    /// Srmount error.
    Esrmnt = 69,
    /// Communication error on send.
    Ecomm = 70,
    /// Protocol error.
    Eproto = 71,
    /// Multihop attempted.
    Emultihop = 72,
    /// RFS specific error.
    Edotdot = 73,
    /// Not a data message.
    Ebadmsg = 74,
    /// Value too large for defined data type.
    Eoverflow = 75,
    /// Name not unique on network.
    Enotuniq = 76,
    /// File descriptor in bad state.
    Ebadfd = 77,
    /// Remote address changed.
    Eremchg = 78,
    /// Can not access a needed shared library.
    Elibacc = 79,
    /// Accessing a corrupted shared library.
    Elibbad = 80,
    /// .lib section in a.out corrupted.
    Elibscn = 81,
}

impl Error {
    /// Returns the name of the error.
    pub fn name(&self) -> &'static str {
        match self {
            Error::Eperm => "Eperm",
            Error::Enoent => "Enoent",
            Error::Esrch => "Esrch",
            Error::Eintr => "Eintr",
            Error::Eio => "Eio",
            Error::Enxio => "Enxio",
            Error::E2big => "E2big",
            Error::Enoexec => "Enoexec",
            Error::Ebadf => "Ebadf",
            Error::Echild => "Echild",
            Error::Eagain => "Eagain",
            Error::Enomem => "Enomem",
            Error::Eacces => "Eacces",
            Error::Efault => "Efault",
            Error::Enotblk => "Enotblk",
            Error::Ebusy => "Ebusy",
            Error::Eexist => "Eexist",
            Error::Exdev => "Exdev",
            Error::Enodev => "Enodev",
            Error::Enotdir => "Enotdir",
            Error::Eisdir => "Eisdir",
            Error::Einval => "Einval",
            Error::Enfile => "Enfile",
            Error::Emfile => "Emfile",
            _ => "Unknown",
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A [`Result`] with an [`Error`] error variant.
///
///
/// ```
/// fn example() -> Result {
///     Err(Error::Eperm)
/// }
/// ```
///
pub type Result<T = (), E = Error> = core::result::Result<T, E>;
