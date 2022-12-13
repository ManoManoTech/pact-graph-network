pub mod fs {
    use std::{fs as stdFs, io, path::Path};

    pub fn write<P, C>(path: P, content: C) -> io::Result<()>
    where
        P: AsRef<Path> + 'static,
        C: AsRef<[u8]> + 'static,
    {
        stdFs::write(path, content)
    }
}
