use std::fs;

fn main() -> Result<(), std::io::Error> {
    let mut intentional_invalid_magic =
        *include_bytes!("../../../class_basket/intentional_invalid_magic.class");

    intentional_invalid_magic[0] = 0;
    intentional_invalid_magic[1] = 0;
    intentional_invalid_magic[2] = 0;
    intentional_invalid_magic[3] = 0;

    fs::write(
        "../../../class_basket/intentional_invalid_magic.class",
        intentional_invalid_magic,
    )?;

    println!("[Log] -> Made class file containing invalid magic number.");

    Ok(())
}
