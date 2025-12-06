#![forbid(unsafe_code, future_incompatible)]
#![deny(unused)]

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Goblin(#[from] goblin::error::Error),
    #[error("debug data unavailable")]
    NoDebugData,
    #[error("pdb info unavailable")]
    NoPdbInfo,
}

pub fn generate_link(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    let file = std::fs::read(path)?;
    let pe = goblin::pe::PE::parse(&file)?;

    let goblin::pe::debug::CodeviewPDB70DebugInfo {
        filename,
        age,
        signature,
        ..
    } = pe
        .debug_data
        .ok_or(Error::NoDebugData)?
        .codeview_pdb70_debug_info
        .ok_or(Error::NoPdbInfo)?;

    let filename = std::str::from_utf8(filename.split(|&c| c == b'\\').next_back().unwrap())
        .unwrap()
        .trim_end_matches('\0');
    let signature = format_uuid(signature);

    Ok(format!(
        "http://msdl.microsoft.com/download/symbols/{filename}/{signature}{age:X}/{filename}"
    ))
}

fn format_uuid(uuid: [u8; 16]) -> String {
    uuid[0..4]
        .iter()
        .rev()
        .chain(uuid[4..6].iter().rev())
        .chain(uuid[6..8].iter().rev())
        .chain(&uuid[8..])
        .map(|b| format!("{b:0>2X}"))
        .collect()
}
