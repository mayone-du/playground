use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir("./example/src/pages")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    println!("{:#?}", entries);

    // The entries have now been sorted by their path.

    Ok(())
}
