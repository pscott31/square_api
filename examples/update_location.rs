#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let response = client
        .update_location(location_id)
        .location(Location {
            address: Some(Address {
                address_line1: Some("your address line 1".to_owned()),
                address_line2: Some("your address line 2".to_owned()),
                address_line3: Some("your address line 3".to_owned()),
                administrative_district_level1: Some(
                    "your administrative district level 1".to_owned(),
                ),
                administrative_district_level2: Some(
                    "your administrative district level 2".to_owned(),
                ),
                administrative_district_level3: Some(
                    "your administrative district level 3".to_owned(),
                ),
                country: Some("your country".to_owned()),
                first_name: Some("your first name".to_owned()),
                last_name: Some("your last name".to_owned()),
                locality: Some("your locality".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                sublocality: Some("your sublocality".to_owned()),
                sublocality2: Some("your sublocality 2".to_owned()),
                sublocality3: Some("your sublocality 3".to_owned()),
            }),
            business_email: Some("your business email".to_owned()),
            business_hours: Some(BusinessHours {
                periods: Some(
                    vec![
                        BusinessHoursPeriod { day_of_week : Some("your day of week"
                        .to_owned()), end_local_time : Some("your end local time"
                        .to_owned()), start_local_time : Some("your start local time"
                        .to_owned()) }
                    ],
                ),
            }),
            business_name: Some("your business name".to_owned()),
            capabilities: Some(vec!["your capabilities".to_owned()]),
            coordinates: Some(Coordinates {
                latitude: Some(1.0),
                longitude: Some(1.0),
            }),
            country: Some("your country".to_owned()),
            created_at: Some("your created at".to_owned()),
            currency: Some("your currency".to_owned()),
            description: Some("your description".to_owned()),
            facebook_url: Some("your facebook url".to_owned()),
            full_format_logo_url: Some("your full format logo url".to_owned()),
            id: Some("your id".to_owned()),
            instagram_username: Some("your instagram username".to_owned()),
            language_code: Some("your language code".to_owned()),
            logo_url: Some("your logo url".to_owned()),
            mcc: Some("your mcc".to_owned()),
            merchant_id: Some("your merchant id".to_owned()),
            name: Some("your name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
            pos_background_url: Some("your pos background url".to_owned()),
            status: Some("your status".to_owned()),
            tax_ids: Some(TaxIds {
                es_nif: Some("your es nif".to_owned()),
                eu_vat: Some("your eu vat".to_owned()),
                fr_naf: Some("your fr naf".to_owned()),
                fr_siret: Some("your fr siret".to_owned()),
            }),
            timezone: Some("your timezone".to_owned()),
            twitter_username: Some("your twitter username".to_owned()),
            type_: Some("your type".to_owned()),
            website_url: Some("your website url".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}