quick_error! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum MemoryConfigurationReadError {
        NoConfiguration {
            description("no configuration")
        }
    }
}