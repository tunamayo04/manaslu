#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::log::init_logging;
    use crate::manaslu::Manaslu;

    #[test]
    fn nesttest_test_rom() -> Result<(), Box<dyn std::error::Error>> {
        init_logging();

        // fn it_works() {
        let mut manaslu = Manaslu::new(PathBuf::from("../roms/nestest.nes"))?;

        manaslu.run();

        Ok(())
     }
}