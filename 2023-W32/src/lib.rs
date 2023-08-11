use std::{env, error::Error, sync::OnceLock};

use reqwest::Client;

pub fn luhn_check(cc_num: usize) -> bool {
    // Split check digit from payload
    let (payload, check_digit) = (cc_num / 10, cc_num % 10);

    let sum =
        // Start from rightmost digit. Moving left, double the value of every second digit.
        payload.to_string().chars().map(|c| c.to_digit(10).unwrap()).rev().enumerate().map(|(i, n)| {
            if i % 2 == 0 {
                let doubled = n * 2;
                // In case of double digits, add them together
                if doubled >= 10 {
                    doubled / 10 + doubled % 10
                } else {
                    doubled
                }
            } else {
                n
            }
        })
        // Sum the values of the resulting digits
        .sum::<u32>() as usize;

    let actual_check_digit = 10 - (sum % 10);
    check_digit == actual_check_digit
}

const IIN_API_URL: &str = "https://api.iinapi.com/iin";
static IIN_API_CLIENT: OnceLock<(Client, String)> = OnceLock::new();

pub async fn iin_lookup(cc_num: usize) -> Result<Option<String>, Box<dyn Error>> {
    if IIN_API_CLIENT.get().is_none() && env::var("IIN_API_KEY").is_err() {
        return Err(
            "API Key missing. Please set IIN_API_KEY environment variable. Sign up at https://iinapi.com if needed.".into(),
        );
    }

    let (client, api_key) =
        IIN_API_CLIENT.get_or_init(|| (Client::new(), env::var("IIN_API_KEY").unwrap()));

    let res = client
        .get(IIN_API_URL)
        .query(&[
            ("key", api_key.as_str()),
            ("digits", &format!("{}", cc_num)[..6]),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    match res.get("result") {
        Some(res) => match res.get("CardBrand").and_then(|val| val.as_str()) {
            Some(brand) => Ok(Some(brand.to_owned())),
            None => Ok(None),
        },
        None => Err("Deserialization error from iin api".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luhn_check_works() {
        assert_eq!(luhn_check(123456789), false);
        assert_eq!(luhn_check(79927398713), true);
        assert_eq!(luhn_check(5555555555554444), true);
    }

    #[tokio::test]
    async fn iin_lookup_works() {
        assert_eq!(
            iin_lookup(5555555555554444).await.unwrap(),
            Some("MASTERCARD".into())
        );
        assert_eq!(
            iin_lookup(4111111111111111).await.unwrap(),
            Some("VISA".into())
        );
    }
}
