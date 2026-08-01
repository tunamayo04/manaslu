#[cfg(test)]
mod tests {
    use crate::log::init_logging;
    use crate::manaslu::Manaslu;

    #[test]
    fn ManasluNew_Run_ShouldWork() -> Result<(), Box<dyn std::error::Error>> {
        init_logging();

        // fn it_works() {
        let mut manaslu = Manaslu::new("src/roms/nestest.nes")?;

        manaslu.run_from_address(0xC000);

        Ok(())
     }
}