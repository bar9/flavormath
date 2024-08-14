#[derive(Debug, PartialEq)]
pub enum Unit {
    Gramm
}

pub fn scale (ingredients: Vec<(String, i32, Unit)>, amount: i32) ->  Vec<(String, i32, Unit)> {
    ingredients
        .into_iter()
        .map(|(name, qty, unit)| (name, qty / amount, unit))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_calculation() {
        let amount = 7;
        let ingredients = vec![
            ("Zucker".to_string(), 210, Unit::Gramm)
        ];
        let result = scale(ingredients, amount);
        assert_eq!(result[0].0, "Zucker".to_string());
        assert_eq!(result[0].1, 30);
        assert_eq!(result[0].2, Unit::Gramm);
    }
}