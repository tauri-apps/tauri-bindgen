use std::{path::Path, fs::File, io, cmp};

const TRUNCATE_LEN: usize = 16;

pub fn hash_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    
    if let Some(mmap) = maybe_memmap_file(&file)? {
        let cursor = io::Cursor::new(mmap);
        hasher.update_rayon(cursor.get_ref());
    } else {
        copy_wide(file, &mut hasher)?;
    }
    
    let output = hasher.finalize_xof();
    Ok(encode_hex(output))
}

pub fn hash_string(str: impl AsRef<str>) -> anyhow::Result<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(str.as_ref().as_bytes());

    let output = hasher.finalize_xof();
    Ok(encode_hex(output))
}


// Mmap a file, if it looks like a good idea. Return None in cases where we
// know mmap will fail, or if the file is short enough that mmapping isn't
// worth it. However, if we do try to mmap and it fails, return the error.
fn maybe_memmap_file(file: &File) -> anyhow::Result<Option<memmap2::Mmap>> {
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    Ok(if !metadata.is_file() {
        // Not a real file.
        None
    } else if file_size > isize::max_value() as u64 {
        // Too long to safely map.
        // https://github.com/danburkert/memmap-rs/issues/69
        None
    } else if file_size == 0 {
        // Mapping an empty file currently fails.
        // https://github.com/danburkert/memmap-rs/issues/72
        None
    } else if file_size < 16 * 1024 {
        // Mapping small files is not worth it.
        None
    } else {
        // Explicitly set the length of the memory map, so that filesystem
        // changes can't race to violate the invariants we just checked.
        let map = unsafe {
            memmap2::MmapOptions::new()
                .len(file_size as usize)
                .map(file)?
        };
        Some(map)
    })
}

fn copy_wide(mut reader: impl io::Read, hasher: &mut blake3::Hasher) -> io::Result<u64> {
    let mut buffer = [0; 65536];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

fn encode_hex(mut output: blake3::OutputReader) -> String {
    // Encoding multiples of the block size is most efficient.
    let mut len = TRUNCATE_LEN;
    let mut block = [0; blake3::guts::BLOCK_LEN];
    let mut str = String::new();

    while len > 0 {
        output.fill(&mut block);
        let hex_str = hex::encode(&block[..]);
        let take_bytes = cmp::min(len, block.len());

        str.push_str(&hex_str[..2 * take_bytes as usize]);
        len -= take_bytes;
    }
    
    str
}