/// SPI NOR stores the AIC header at the beginning of the raw boot image.
pub fn build(aic: &[u8]) -> Vec<u8> {
    aic.to_vec()
}
