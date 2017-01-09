/// The trait for types which are a valid target for configuration to be applied.
pub trait ConfigurationTarget {
    /// The type of the configuration.
    type Configuration;

    /// Applies the specified configuration.
    fn apply(&mut self, configuration: &Self::Configuration);
}