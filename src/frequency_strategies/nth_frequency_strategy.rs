use super::frequency_strategy::FrequencyStrategy;
use super::frequency_strategy_type::FrequencyStrategyType;

pub struct NthFrequencyStrategy {
    target_count: usize,
    counter: usize,
}

impl NthFrequencyStrategy {
    /// Creates a new `NthFrequencyStrategy`.
    ///
    /// # Parameters
    /// - `nth`: The target count for the strategy.
    ///
    /// # Returns
    /// A new `NthFrequencyStrategy` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seek::frequency_strategies::nth_frequency_strategy::NthFrequencyStrategy;
    ///
    /// let strategy = NthFrequencyStrategy::new(3);
    /// ```
    pub fn new(nth: usize) -> Self {
        Self {
            target_count: nth,
            counter: 0,
        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
}

impl FrequencyStrategy for NthFrequencyStrategy {
    fn strategy_type(&self) -> FrequencyStrategyType {
        FrequencyStrategyType::Nth
    }

    fn matches_frequency(&mut self) -> bool {
        self.increment_counter();
        self.counter == self.target_count
    }

    fn is_done(&self) -> bool {
        self.counter == self.target_count
    }
}
