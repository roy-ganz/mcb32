use stm32f1xx_hal::serial;

pub struct BoardConfig {
    pub(crate) uart1: serial::Config,
    pub(crate) uart2: serial::Config

}

impl BoardConfig {

    pub fn uart1(mut self, config: serial::Config) -> Self{
        self.uart1 = config;
        self
    }
    pub fn uart2(mut self, config: serial::Config) -> Self{
        self.uart1 = config;
        self
    }

}

impl Default for BoardConfig {
    fn default() -> Self { 
        BoardConfig{
            uart1: serial::Config::default(),
            uart2: serial::Config::default()
        }
     }
}