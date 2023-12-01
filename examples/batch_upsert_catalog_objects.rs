#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let batches = vec![
        CatalogObjectBatch { objects : vec![CatalogObject { absent_at_location_ids :
        Some(vec!["your absent at location ids".to_owned()]), catalog_v1_ids :
        Some(vec![CatalogV1Id { catalog_v1_id : Some("your catalog v 1 id".to_owned()),
        location_id : Some("your location id".to_owned()) }]), category_data :
        Some(CatalogCategory { image_ids : Some(vec!["your image ids".to_owned()]), name
        : Some("your name".to_owned()) }), custom_attribute_definition_data :
        Some(CatalogCustomAttributeDefinition { allowed_object_types :
        vec!["your allowed object types".to_owned()], app_visibility :
        Some("your app visibility".to_owned()), custom_attribute_usage_count : Some(1),
        description : Some("your description".to_owned()), key : Some("your key"
        .to_owned()), name : "your name".to_owned(), number_config :
        Some(CatalogCustomAttributeDefinitionNumberConfig { precision : Some(1) }),
        selection_config : Some(CatalogCustomAttributeDefinitionSelectionConfig {
        allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : Some(CatalogItem { abbreviation :
        Some("your abbreviation".to_owned()), available_electronically : Some(true),
        available_for_pickup : Some(true), available_online : Some(true), category_id :
        Some("your category id".to_owned()), description : Some("your description"
        .to_owned()), description_html : Some("your description html".to_owned()),
        description_plaintext : Some("your description plaintext".to_owned()), image_ids
        : Some(vec!["your image ids".to_owned()]), image_url : Some("your image url"
        .to_owned()), item_options : Some(vec![CatalogItemOptionForItem { item_option_id
        : Some("your item option id".to_owned()) }]), label_color :
        Some("your label color".to_owned()), modifier_list_info :
        Some(vec![CatalogItemModifierListInfo { enabled : Some(true),
        max_selected_modifiers : Some(1), min_selected_modifiers : Some(1),
        modifier_list_id : "your modifier list id".to_owned(), modifier_overrides :
        Some(vec![CatalogModifierOverride { modifier_id : "your modifier id".to_owned(),
        on_by_default : Some(true) }]) }]), name : Some("your name".to_owned()),
        product_type : Some("your product type".to_owned()), skip_modifier_screen :
        Some(true), sort_name : Some("your sort name".to_owned()), tax_ids :
        Some(vec!["your tax ids".to_owned()]), variations : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data :
        Some(CatalogItemOption { description : Some("your description".to_owned()),
        display_name : Some("your display name".to_owned()), name : Some("your name"
        .to_owned()), show_colors : Some(true), values : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : Some(CatalogModifierList { image_ids :
        Some(vec!["your image ids".to_owned()]), modifiers : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]), name : Some("your name".to_owned()), ordinal
        : Some(1), selection_type : Some("your selection type".to_owned()) }),
        present_at_all_locations : Some(true), present_at_location_ids :
        Some(vec!["your present at location ids".to_owned()]), pricing_rule_data :
        Some(CatalogPricingRule { apply_products_id : Some("your apply products id"
        .to_owned()), customer_group_ids_any : Some(vec!["your customer group ids any"
        .to_owned()]), discount_id : Some("your discount id".to_owned()),
        exclude_products_id : Some("your exclude products id".to_owned()),
        exclude_strategy : Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_value_data :
        Some(CatalogItemOptionValue { color : Some("your color".to_owned()), description
        : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : Some(CatalogModifierList { image_ids :
        Some(vec!["your image ids".to_owned()]), modifiers : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data :
        Some(CatalogItemOption { description : Some("your description".to_owned()),
        display_name : Some("your display name".to_owned()), name : Some("your name"
        .to_owned()), show_colors : Some(true), values : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_value_data :
        Some(CatalogItemOptionValue { color : Some("your color".to_owned()), description
        : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]), name : Some("your name".to_owned()), ordinal
        : Some(1), selection_type : Some("your selection type".to_owned()) }),
        present_at_all_locations : Some(true), present_at_location_ids :
        Some(vec!["your present at location ids".to_owned()]), pricing_rule_data :
        Some(CatalogPricingRule { apply_products_id : Some("your apply products id"
        .to_owned()), customer_group_ids_any : Some(vec!["your customer group ids any"
        .to_owned()]), discount_id : Some("your discount id".to_owned()),
        exclude_products_id : Some("your exclude products id".to_owned()),
        exclude_strategy : Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_data : Some(CatalogItemOption
        { description : Some("your description".to_owned()), display_name :
        Some("your display name".to_owned()), name : Some("your name".to_owned()),
        show_colors : Some(true), values : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : Some(CatalogItem { abbreviation :
        Some("your abbreviation".to_owned()), available_electronically : Some(true),
        available_for_pickup : Some(true), available_online : Some(true), category_id :
        Some("your category id".to_owned()), description : Some("your description"
        .to_owned()), description_html : Some("your description html".to_owned()),
        description_plaintext : Some("your description plaintext".to_owned()), image_ids
        : Some(vec!["your image ids".to_owned()]), image_url : Some("your image url"
        .to_owned()), item_options : Some(vec![CatalogItemOptionForItem { item_option_id
        : Some("your item option id".to_owned()) }]), label_color :
        Some("your label color".to_owned()), modifier_list_info :
        Some(vec![CatalogItemModifierListInfo { enabled : Some(true),
        max_selected_modifiers : Some(1), min_selected_modifiers : Some(1),
        modifier_list_id : "your modifier list id".to_owned(), modifier_overrides :
        Some(vec![CatalogModifierOverride { modifier_id : "your modifier id".to_owned(),
        on_by_default : Some(true) }]) }]), name : Some("your name".to_owned()),
        product_type : Some("your product type".to_owned()), skip_modifier_screen :
        Some(true), sort_name : Some("your sort name".to_owned()), tax_ids :
        Some(vec!["your tax ids".to_owned()]), variations : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : Some(CatalogModifierList { image_ids :
        Some(vec!["your image ids".to_owned()]), modifiers : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]), name : Some("your name".to_owned()), ordinal
        : Some(1), selection_type : Some("your selection type".to_owned()) }),
        present_at_all_locations : Some(true), present_at_location_ids :
        Some(vec!["your present at location ids".to_owned()]), pricing_rule_data :
        Some(CatalogPricingRule { apply_products_id : Some("your apply products id"
        .to_owned()), customer_group_ids_any : Some(vec!["your customer group ids any"
        .to_owned()]), discount_id : Some("your discount id".to_owned()),
        exclude_products_id : Some("your exclude products id".to_owned()),
        exclude_strategy : Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : Some(CatalogModifierList { image_ids :
        Some(vec!["your image ids".to_owned()]), modifiers : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : Some(CatalogItem { abbreviation :
        Some("your abbreviation".to_owned()), available_electronically : Some(true),
        available_for_pickup : Some(true), available_online : Some(true), category_id :
        Some("your category id".to_owned()), description : Some("your description"
        .to_owned()), description_html : Some("your description html".to_owned()),
        description_plaintext : Some("your description plaintext".to_owned()), image_ids
        : Some(vec!["your image ids".to_owned()]), image_url : Some("your image url"
        .to_owned()), item_options : Some(vec![CatalogItemOptionForItem { item_option_id
        : Some("your item option id".to_owned()) }]), label_color :
        Some("your label color".to_owned()), modifier_list_info :
        Some(vec![CatalogItemModifierListInfo { enabled : Some(true),
        max_selected_modifiers : Some(1), min_selected_modifiers : Some(1),
        modifier_list_id : "your modifier list id".to_owned(), modifier_overrides :
        Some(vec![CatalogModifierOverride { modifier_id : "your modifier id".to_owned(),
        on_by_default : Some(true) }]) }]), name : Some("your name".to_owned()),
        product_type : Some("your product type".to_owned()), skip_modifier_screen :
        Some(true), sort_name : Some("your sort name".to_owned()), tax_ids :
        Some(vec!["your tax ids".to_owned()]), variations : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]), name : Some("your name".to_owned()), ordinal
        : Some(1), selection_type : Some("your selection type".to_owned()) }),
        present_at_all_locations : Some(true), present_at_location_ids :
        Some(vec!["your present at location ids".to_owned()]), pricing_rule_data :
        Some(CatalogPricingRule { apply_products_id : Some("your apply products id"
        .to_owned()), customer_group_ids_any : Some(vec!["your customer group ids any"
        .to_owned()]), discount_id : Some("your discount id".to_owned()),
        exclude_products_id : Some("your exclude products id".to_owned()),
        exclude_strategy : Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_value_data :
        Some(CatalogItemOptionValue { color : Some("your color".to_owned()), description
        : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : Some(CatalogModifierList { image_ids :
        Some(vec!["your image ids".to_owned()]), modifiers : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : Some(CatalogItem { abbreviation :
        Some("your abbreviation".to_owned()), available_electronically : Some(true),
        available_for_pickup : Some(true), available_online : Some(true), category_id :
        Some("your category id".to_owned()), description : Some("your description"
        .to_owned()), description_html : Some("your description html".to_owned()),
        description_plaintext : Some("your description plaintext".to_owned()), image_ids
        : Some(vec!["your image ids".to_owned()]), image_url : Some("your image url"
        .to_owned()), item_options : Some(vec![CatalogItemOptionForItem { item_option_id
        : Some("your item option id".to_owned()) }]), label_color :
        Some("your label color".to_owned()), modifier_list_info :
        Some(vec![CatalogItemModifierListInfo { enabled : Some(true),
        max_selected_modifiers : Some(1), min_selected_modifiers : Some(1),
        modifier_list_id : "your modifier list id".to_owned(), modifier_overrides :
        Some(vec![CatalogModifierOverride { modifier_id : "your modifier id".to_owned(),
        on_by_default : Some(true) }]) }]), name : Some("your name".to_owned()),
        product_type : Some("your product type".to_owned()), skip_modifier_screen :
        Some(true), sort_name : Some("your sort name".to_owned()), tax_ids :
        Some(vec!["your tax ids".to_owned()]), variations : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data :
        Some(CatalogItemOption { description : Some("your description".to_owned()),
        display_name : Some("your display name".to_owned()), name : Some("your name"
        .to_owned()), show_colors : Some(true), values : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_value_data :
        Some(CatalogItemOptionValue { color : Some("your color".to_owned()), description
        : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_data : Some(CatalogItemOption
        { description : Some("your description".to_owned()), display_name :
        Some("your display name".to_owned()), name : Some("your name".to_owned()),
        show_colors : Some(true), values : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : Some(CatalogItem { abbreviation :
        Some("your abbreviation".to_owned()), available_electronically : Some(true),
        available_for_pickup : Some(true), available_online : Some(true), category_id :
        Some("your category id".to_owned()), description : Some("your description"
        .to_owned()), description_html : Some("your description html".to_owned()),
        description_plaintext : Some("your description plaintext".to_owned()), image_ids
        : Some(vec!["your image ids".to_owned()]), image_url : Some("your image url"
        .to_owned()), item_options : Some(vec![CatalogItemOptionForItem { item_option_id
        : Some("your item option id".to_owned()) }]), label_color :
        Some("your label color".to_owned()), modifier_list_info :
        Some(vec![CatalogItemModifierListInfo { enabled : Some(true),
        max_selected_modifiers : Some(1), min_selected_modifiers : Some(1),
        modifier_list_id : "your modifier list id".to_owned(), modifier_overrides :
        Some(vec![CatalogModifierOverride { modifier_id : "your modifier id".to_owned(),
        on_by_default : Some(true) }]) }]), name : Some("your name".to_owned()),
        product_type : Some("your product type".to_owned()), skip_modifier_screen :
        Some(true), sort_name : Some("your sort name".to_owned()), tax_ids :
        Some(vec!["your tax ids".to_owned()]), variations : Some(vec![CatalogObject {
        absent_at_location_ids : Some(vec!["your absent at location ids".to_owned()]),
        catalog_v1_ids : Some(vec![CatalogV1Id { catalog_v1_id :
        Some("your catalog v 1 id".to_owned()), location_id : Some("your location id"
        .to_owned()) }]), category_data : Some(CatalogCategory { image_ids :
        Some(vec!["your image ids".to_owned()]), name : Some("your name".to_owned()) }),
        custom_attribute_definition_data : Some(CatalogCustomAttributeDefinition {
        allowed_object_types : vec!["your allowed object types".to_owned()],
        app_visibility : Some("your app visibility".to_owned()),
        custom_attribute_usage_count : Some(1), description : Some("your description"
        .to_owned()), key : Some("your key".to_owned()), name : "your name".to_owned(),
        number_config : Some(CatalogCustomAttributeDefinitionNumberConfig { precision :
        Some(1) }), selection_config :
        Some(CatalogCustomAttributeDefinitionSelectionConfig { allowed_selections :
        Some(vec![CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection
        { name : "your name".to_owned(), uid : Some("your uid".to_owned()) }]),
        max_allowed_selections : Some(1) }), seller_visibility :
        Some("your seller visibility".to_owned()), source_application :
        Some(SourceApplication { application_id : Some("your application id".to_owned()),
        name : Some("your name".to_owned()), product : Some("your product".to_owned())
        }), string_config : Some(CatalogCustomAttributeDefinitionStringConfig {
        enforce_uniqueness : Some(true) }), type_ : "your type".to_owned() }),
        custom_attribute_values : Some(serde_json::json!({})), discount_data :
        Some(CatalogDiscount { amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), discount_type : Some("your discount type"
        .to_owned()), label_color : Some("your label color".to_owned()),
        maximum_amount_money : Some(Money { amount : Some(1), currency :
        Some("your currency".to_owned()) }), modify_tax_basis :
        Some("your modify tax basis".to_owned()), name : Some("your name".to_owned()),
        percentage : Some("your percentage".to_owned()), pin_required : Some(true) }), id
        : "your id".to_owned(), image_data : Some(CatalogImage { caption :
        Some("your caption".to_owned()), name : Some("your name".to_owned()),
        photo_studio_order_id : Some("your photo studio order id".to_owned()), url :
        Some("your url".to_owned()) }), image_id : Some("your image id".to_owned()),
        is_deleted : Some(true), item_data : None, item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_data : None,
        item_option_value_data : Some(CatalogItemOptionValue { color : Some("your color"
        .to_owned()), description : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]) }), item_option_value_data :
        Some(CatalogItemOptionValue { color : Some("your color".to_owned()), description
        : Some("your description".to_owned()), item_option_id :
        Some("your item option id".to_owned()), name : Some("your name".to_owned()),
        ordinal : Some(1) }), item_variation_data : Some(CatalogItemVariation {
        available_for_booking : Some(true), image_ids : Some(vec!["your image ids"
        .to_owned()]), inventory_alert_threshold : Some(1), inventory_alert_type :
        Some("your inventory alert type".to_owned()), item_id : Some("your item id"
        .to_owned()), item_option_values :
        Some(vec![CatalogItemOptionValueForItemVariation { item_option_id :
        Some("your item option id".to_owned()), item_option_value_id :
        Some("your item option value id".to_owned()) }]), location_overrides :
        Some(vec![ItemVariationLocationOverrides { inventory_alert_threshold : Some(1),
        inventory_alert_type : Some("your inventory alert type".to_owned()), location_id
        : Some("your location id".to_owned()), price_money : Some(Money { amount :
        Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sold_out : Some(true), sold_out_valid_until
        : Some("your sold out valid until".to_owned()), track_inventory : Some(true) }]),
        measurement_unit_id : Some("your measurement unit id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }), pricing_type :
        Some("your pricing type".to_owned()), sellable : Some(true), service_duration :
        Some(1), sku : Some("your sku".to_owned()), stockable : Some(true),
        stockable_conversion : Some(CatalogStockConversion { nonstockable_quantity :
        "your nonstockable quantity".to_owned(), stockable_item_variation_id :
        "your stockable item variation id".to_owned(), stockable_quantity :
        "your stockable quantity".to_owned() }), team_member_ids :
        Some(vec!["your team member ids".to_owned()]), track_inventory : Some(true), upc
        : Some("your upc".to_owned()), user_data : Some("your user data".to_owned()) }),
        measurement_unit_data : Some(CatalogMeasurementUnit { measurement_unit :
        Some(MeasurementUnit { area_unit : Some("your area unit".to_owned()), custom_unit
        : Some(MeasurementUnitCustom { abbreviation : "your abbreviation".to_owned(),
        name : "your name".to_owned() }), generic_unit : Some("your generic unit"
        .to_owned()), length_unit : Some("your length unit".to_owned()), time_unit :
        Some("your time unit".to_owned()), type_ : Some("your type".to_owned()),
        volume_unit : Some("your volume unit".to_owned()), weight_unit :
        Some("your weight unit".to_owned()) }), precision : Some(1) }), modifier_data :
        Some(CatalogModifier { image_ids : Some(vec!["your image ids".to_owned()]),
        modifier_list_id : Some("your modifier list id".to_owned()), name :
        Some("your name".to_owned()), ordinal : Some(1), price_money : Some(Money {
        amount : Some(1), currency : Some("your currency".to_owned()) }) }),
        modifier_list_data : None, present_at_all_locations : Some(true),
        present_at_location_ids : Some(vec!["your present at location ids".to_owned()]),
        pricing_rule_data : Some(CatalogPricingRule { apply_products_id :
        Some("your apply products id".to_owned()), customer_group_ids_any :
        Some(vec!["your customer group ids any".to_owned()]), discount_id :
        Some("your discount id".to_owned()), exclude_products_id :
        Some("your exclude products id".to_owned()), exclude_strategy :
        Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }]), name : Some("your name".to_owned()), ordinal
        : Some(1), selection_type : Some("your selection type".to_owned()) }),
        present_at_all_locations : Some(true), present_at_location_ids :
        Some(vec!["your present at location ids".to_owned()]), pricing_rule_data :
        Some(CatalogPricingRule { apply_products_id : Some("your apply products id"
        .to_owned()), customer_group_ids_any : Some(vec!["your customer group ids any"
        .to_owned()]), discount_id : Some("your discount id".to_owned()),
        exclude_products_id : Some("your exclude products id".to_owned()),
        exclude_strategy : Some("your exclude strategy".to_owned()), match_products_id :
        Some("your match products id".to_owned()), minimum_order_subtotal_money :
        Some(Money { amount : Some(1), currency : Some("your currency".to_owned()) }),
        name : Some("your name".to_owned()), time_period_ids :
        Some(vec!["your time period ids".to_owned()]), valid_from_date :
        Some("your valid from date".to_owned()), valid_from_local_time :
        Some("your valid from local time".to_owned()), valid_until_date :
        Some("your valid until date".to_owned()), valid_until_local_time :
        Some("your valid until local time".to_owned()) }), product_set_data :
        Some(CatalogProductSet { all_products : Some(true), name : Some("your name"
        .to_owned()), product_ids_all : Some(vec!["your product ids all".to_owned()]),
        product_ids_any : Some(vec!["your product ids any".to_owned()]), quantity_exact :
        Some(1), quantity_max : Some(1), quantity_min : Some(1) }),
        quick_amounts_settings_data : Some(CatalogQuickAmountsSettings { amounts :
        Some(vec![CatalogQuickAmount { amount : Money { amount : None, currency :
        Some("your currency".to_owned()) }, ordinal : Some(1), score : Some(1), type_ :
        "your type".to_owned() }]), eligible_for_auto_amounts : Some(true), option :
        "your option".to_owned() }), subscription_plan_data :
        Some(CatalogSubscriptionPlan { name : "your name".to_owned(), phases :
        vec![SubscriptionPhase { cadence : "your cadence".to_owned(), ordinal : Some(1),
        periods : Some(1), recurring_price_money : Some(Money { amount : Some(1),
        currency : Some("your currency".to_owned()) }), uid : Some("your uid".to_owned())
        }] }), tax_data : Some(CatalogTax { applies_to_custom_amounts : Some(true),
        calculation_phase : Some("your calculation phase".to_owned()), enabled :
        Some(true), inclusion_type : Some("your inclusion type".to_owned()), name :
        Some("your name".to_owned()), percentage : Some("your percentage".to_owned()) }),
        time_period_data : Some(CatalogTimePeriod { event : Some("your event".to_owned())
        }), type_ : "your type".to_owned(), updated_at : Some("your updated at"
        .to_owned()), version : Some(1) }] }
    ];
    let idempotency_key = "your idempotency key";
    let response = client
        .batch_upsert_catalog_objects(batches, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}