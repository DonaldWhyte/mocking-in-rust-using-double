pub trait ProfitForecaster {
    fn profit_at(&self, timestamp: i32) -> f64;
}

pub fn forecast_profit_over_time<F: ProfitForecaster>(forecaster: &F,
                                                      start: i32,
                                                      end: i32) -> Vec<f64>
{
    (start..end)
        .map(|t| forecaster.profit_at(t))
        .collect()
}

#[cfg(test)]
mod tests {
    extern crate double;
    use super::*;

    // Generate mock implementations of collaborators
    mock_trait!(
        MockForecaster,
        profit_at(i32) -> f64);
    impl ProfitForecaster for MockForecaster {
        mock_method!(profit_at(&self, timestamp: i32) -> f64);
    }

    #[test]
    fn no_return_value_specified() {
        // GIVEN:
        let forecaster = MockForecaster::default();

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        // default value of return type is used if no value is specified
        assert_eq!(vec!(0.0, 0.0, 0.0), profit_over_time);
    }

    #[test]
    fn single_return_value() {
        // GIVEN:
        let forecaster = MockForecaster::default();
        forecaster.profit_at.return_value(10);

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        assert_eq!(vec!(10.0, 10.0, 10.0), profit_over_time);
    }

    #[test]
    fn multiple_return_values() {
        // GIVEN:
        let forecaster = MockForecaster::default();
        forecaster.profit_at.return_values(vec!(1, 5, 10));

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        assert_eq!(vec!(1.0, 5.0, 10.0), profit_over_time);
    }

    #[test]
    fn return_value_for_specific_arguments() {
        // GIVEN:
        let forecaster = MockForecaster::default();
        forecaster.profit_at.return_value(10);
        forecaster.profit_at.return_value_for((1), 5);

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        assert_eq!(vec!(10.0, 5.0, 10.0), profit_over_time);
    }

    #[test]
    fn using_closure_to_compute_return_value() {
        // GIVEN:
        let forecaster = MockForecaster::default();
        forecaster.profit_at.use_closure(
            Box::new(|t| t as f64 * 5.0 + 1.0));

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        assert_eq!(vec!(1.0, 6.0, 11.0), profit_over_time);
    }

    #[test]
    fn using_closure_for_specific_return_value() {
        // GIVEN:
        let forecaster = MockForecaster::default();
        forecaster.profit_at.return_value(10);
        forecaster.profit_at.use_closure_for(
            (2),
            Box::new(|t| t as f64 * 5.0 + 1.0));

        // WHEN:
        let profit_over_time = forecast_profit_over_time(&forecaster, 0, 3);

        // THEN:
        assert_eq!(vec!(10.0, 10.0, 11.0), profit_over_time);
    }

}
