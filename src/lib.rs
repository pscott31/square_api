//! [`SquareApiClient`](struct.SquareApiClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct SquareApiClient {
    pub client: httpclient::Client,
}
impl SquareApiClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://connect.squareup.com"),
        }
    }
}
impl SquareApiClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**RegisterDomain

Activates a domain for use with Apple Pay on the Web and Square. A validation
is performed on this domain by Apple to ensure that it is properly set up as
an Apple Pay enabled domain.

This endpoint provides an easy way for platform developers to bulk activate
Apple Pay on the Web with Square for merchants using their platform.

To learn more about Web Apple Pay, see
[Add the Apple Pay on the Web Button](https://developer.squareup.com/docs/payment-form/add-digital-wallets/apple-pay).*/
    pub fn register_domain(&self, domain_name: &str) -> request::RegisterDomainRequest {
        request::RegisterDomainRequest {
            http_client: &self,
            domain_name: domain_name.to_owned(),
        }
    }
    /**ListBankAccounts

Returns a list of [BankAccount](entity:BankAccount) objects linked to a Square account.

See endpoint docs at <https://developer.squareup.com/docs/bank-accounts-api#retrieve-all-bank-accounts-linked-to-a-square-account>.*/
    pub fn list_bank_accounts(&self) -> request::ListBankAccountsRequest {
        request::ListBankAccountsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: None,
        }
    }
    /**GetBankAccountByV1Id

Returns details of a [BankAccount](entity:BankAccount) identified by V1 bank account ID.

See endpoint docs at <https://developer.squareup.com/docs/bank-accounts-api#retrieve-a-bank-account-by-using-an-id-issued-by-the-v1-bank-accounts-api>.*/
    pub fn get_bank_account_by_v1_id(
        &self,
        v1_bank_account_id: &str,
    ) -> request::GetBankAccountByV1IdRequest {
        request::GetBankAccountByV1IdRequest {
            http_client: &self,
            v1_bank_account_id: v1_bank_account_id.to_owned(),
        }
    }
    /**GetBankAccount

Returns details of a [BankAccount](entity:BankAccount)
linked to a Square account.

See endpoint docs at <https://developer.squareup.com/docs/bank-accounts-api#retrieve-a-bank-account-by-id>.*/
    pub fn get_bank_account(
        &self,
        bank_account_id: &str,
    ) -> request::GetBankAccountRequest {
        request::GetBankAccountRequest {
            http_client: &self,
            bank_account_id: bank_account_id.to_owned(),
        }
    }
    /**ListBookings

Retrieve a collection of bookings.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.*/
    pub fn list_bookings(&self) -> request::ListBookingsRequest {
        request::ListBookingsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: None,
            start_at_max: None,
            start_at_min: None,
            team_member_id: None,
        }
    }
    /**CreateBooking

Creates a booking.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.

For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus*
or *Appointments Premium*.*/
    pub fn create_booking(&self, booking: Booking) -> request::CreateBookingRequest {
        request::CreateBookingRequest {
            http_client: &self,
            booking,
            idempotency_key: None,
        }
    }
    /**SearchAvailability

Searches for availabilities for booking.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.*/
    pub fn search_availability(
        &self,
        query: SearchAvailabilityQuery,
    ) -> request::SearchAvailabilityRequest {
        request::SearchAvailabilityRequest {
            http_client: &self,
            query,
        }
    }
    /**RetrieveBusinessBookingProfile

Retrieves a seller's booking profile.*/
    pub fn retrieve_business_booking_profile(
        &self,
    ) -> request::RetrieveBusinessBookingProfileRequest {
        request::RetrieveBusinessBookingProfileRequest {
            http_client: &self,
        }
    }
    /**ListTeamMemberBookingProfiles

Lists booking profiles for team members.*/
    pub fn list_team_member_booking_profiles(
        &self,
    ) -> request::ListTeamMemberBookingProfilesRequest {
        request::ListTeamMemberBookingProfilesRequest {
            http_client: &self,
            bookable_only: None,
            cursor: None,
            limit: None,
            location_id: None,
        }
    }
    /**RetrieveTeamMemberBookingProfile

Retrieves a team member's booking profile.*/
    pub fn retrieve_team_member_booking_profile(
        &self,
        team_member_id: &str,
    ) -> request::RetrieveTeamMemberBookingProfileRequest {
        request::RetrieveTeamMemberBookingProfileRequest {
            http_client: &self,
            team_member_id: team_member_id.to_owned(),
        }
    }
    /**RetrieveBooking

Retrieves a booking.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_READ` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_READ` and `APPOINTMENTS_READ` for the OAuth scope.*/
    pub fn retrieve_booking(&self, booking_id: &str) -> request::RetrieveBookingRequest {
        request::RetrieveBookingRequest {
            http_client: &self,
            booking_id: booking_id.to_owned(),
        }
    }
    /**UpdateBooking

Updates a booking.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.

For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus*
or *Appointments Premium*.*/
    pub fn update_booking(
        &self,
        booking: Booking,
        booking_id: &str,
    ) -> request::UpdateBookingRequest {
        request::UpdateBookingRequest {
            http_client: &self,
            booking,
            booking_id: booking_id.to_owned(),
            idempotency_key: None,
        }
    }
    /**CancelBooking

Cancels an existing booking.

To call this endpoint with buyer-level permissions, set `APPOINTMENTS_WRITE` for the OAuth scope.
To call this endpoint with seller-level permissions, set `APPOINTMENTS_ALL_WRITE` and `APPOINTMENTS_WRITE` for the OAuth scope.

For calls to this endpoint with seller-level permissions to succeed, the seller must have subscribed to *Appointments Plus*
or *Appointments Premium*.*/
    pub fn cancel_booking(&self, booking_id: &str) -> request::CancelBookingRequest {
        request::CancelBookingRequest {
            http_client: &self,
            booking_id: booking_id.to_owned(),
            booking_version: None,
            idempotency_key: None,
        }
    }
    /**ListCards

Retrieves a list of cards owned by the account making the request.
A max of 25 cards will be returned.*/
    pub fn list_cards(&self) -> request::ListCardsRequest {
        request::ListCardsRequest {
            http_client: &self,
            cursor: None,
            customer_id: None,
            include_disabled: None,
            reference_id: None,
            sort_order: None,
        }
    }
    /**CreateCard

Adds a card on file to an existing merchant.*/
    pub fn create_card(
        &self,
        card: Card,
        idempotency_key: &str,
        source_id: &str,
    ) -> request::CreateCardRequest {
        request::CreateCardRequest {
            http_client: &self,
            card,
            idempotency_key: idempotency_key.to_owned(),
            source_id: source_id.to_owned(),
            verification_token: None,
        }
    }
    /**RetrieveCard

Retrieves details for a specific Card.*/
    pub fn retrieve_card(&self, card_id: &str) -> request::RetrieveCardRequest {
        request::RetrieveCardRequest {
            http_client: &self,
            card_id: card_id.to_owned(),
        }
    }
    /**DisableCard

Disables the card, preventing any further updates or charges.
Disabling an already disabled card is allowed but has no effect.*/
    pub fn disable_card(&self, card_id: &str) -> request::DisableCardRequest {
        request::DisableCardRequest {
            http_client: &self,
            card_id: card_id.to_owned(),
        }
    }
    /**ListCashDrawerShifts

Provides the details for all of the cash drawer shifts for a location
in a date range.

See endpoint docs at <https://developer.squareup.com/docs/cashdrawershift-api/reporting#list-cash-drawer-shifts>.*/
    pub fn list_cash_drawer_shifts(
        &self,
        location_id: &str,
    ) -> request::ListCashDrawerShiftsRequest {
        request::ListCashDrawerShiftsRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            end_time: None,
            limit: None,
            location_id: location_id.to_owned(),
            sort_order: None,
        }
    }
    /**RetrieveCashDrawerShift

Provides the summary details for a single cash drawer shift. See
[ListCashDrawerShiftEvents](api-endpoint:CashDrawers-ListCashDrawerShiftEvents) for a list of cash drawer shift events.

See endpoint docs at <https://developer.squareup.com/docs/cashdrawershift-api/reporting#retrieve-a-cash-drawer-shift>.*/
    pub fn retrieve_cash_drawer_shift(
        &self,
        location_id: &str,
        shift_id: &str,
    ) -> request::RetrieveCashDrawerShiftRequest {
        request::RetrieveCashDrawerShiftRequest {
            http_client: &self,
            location_id: location_id.to_owned(),
            shift_id: shift_id.to_owned(),
        }
    }
    /**ListCashDrawerShiftEvents

Provides a paginated list of events for a single cash drawer shift.

See endpoint docs at <https://developer.squareup.com/docs/cashdrawershift-api/reporting#list-cash-drawer-shift-events>.*/
    pub fn list_cash_drawer_shift_events(
        &self,
        location_id: &str,
        shift_id: &str,
    ) -> request::ListCashDrawerShiftEventsRequest {
        request::ListCashDrawerShiftEventsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: location_id.to_owned(),
            shift_id: shift_id.to_owned(),
        }
    }
    /**BatchDeleteCatalogObjects

Deletes a set of [CatalogItem](entity:CatalogItem)s based on the
provided list of target IDs and returns a set of successfully deleted IDs in
the response. Deletion is a cascading event such that all children of the
targeted object are also deleted. For example, deleting a CatalogItem will
also delete all of its [CatalogItemVariation](entity:CatalogItemVariation)
children.

`BatchDeleteCatalogObjects` succeeds even if only a portion of the targeted
IDs can be deleted. The response will only include IDs that were
actually deleted.*/
    pub fn batch_delete_catalog_objects(
        &self,
    ) -> request::BatchDeleteCatalogObjectsRequest {
        request::BatchDeleteCatalogObjectsRequest {
            http_client: &self,
            object_ids: None,
        }
    }
    /**BatchRetrieveCatalogObjects

Returns a set of objects based on the provided ID.
Each [CatalogItem](entity:CatalogItem) returned in the set includes all of its
child information including: all of its
[CatalogItemVariation](entity:CatalogItemVariation) objects, references to
its [CatalogModifierList](entity:CatalogModifierList) objects, and the ids of
any [CatalogTax](entity:CatalogTax) objects that apply to it.*/
    pub fn batch_retrieve_catalog_objects(
        &self,
        object_ids: &[&str],
    ) -> request::BatchRetrieveCatalogObjectsRequest {
        request::BatchRetrieveCatalogObjectsRequest {
            http_client: &self,
            catalog_version: None,
            include_deleted_objects: None,
            include_related_objects: None,
            object_ids: object_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**BatchUpsertCatalogObjects

Creates or updates up to 10,000 target objects based on the provided
list of objects. The target objects are grouped into batches and each batch is
inserted/updated in an all-or-nothing manner. If an object within a batch is
malformed in some way, or violates a database constraint, the entire batch
containing that item will be disregarded. However, other batches in the same
request may still succeed. Each batch may contain up to 1,000 objects, and
batches will be processed in order as long as the total object count for the
request (items, variations, modifier lists, discounts, and taxes) is no more
than 10,000.*/
    pub fn batch_upsert_catalog_objects(
        &self,
        batches: Vec<CatalogObjectBatch>,
        idempotency_key: &str,
    ) -> request::BatchUpsertCatalogObjectsRequest {
        request::BatchUpsertCatalogObjectsRequest {
            http_client: &self,
            batches,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**CreateCatalogImage

Uploads an image file to be represented by a [CatalogImage](entity:CatalogImage) object that can be linked to an existing
[CatalogObject](entity:CatalogObject) instance. The resulting `CatalogImage` is unattached to any `CatalogObject` if the `object_id`
is not specified.

This `CreateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON part and an image file part in
JPEG, PJPEG, PNG, or GIF format. The maximum file size is 15MB.

See endpoint docs at <https://developer.squareup.com/docs/catalog-api/cookbook/create-catalog-image>.*/
    pub fn create_catalog_image(&self) -> request::CreateCatalogImageRequest {
        request::CreateCatalogImageRequest {
            http_client: &self,
        }
    }
    /**UpdateCatalogImage

Uploads a new image file to replace the existing one in the specified [CatalogImage](entity:CatalogImage) object.

This `UpdateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON part and an image file part in
JPEG, PJPEG, PNG, or GIF format. The maximum file size is 15MB.*/
    pub fn update_catalog_image(
        &self,
        image_id: &str,
    ) -> request::UpdateCatalogImageRequest {
        request::UpdateCatalogImageRequest {
            http_client: &self,
            image_id: image_id.to_owned(),
        }
    }
    /**CatalogInfo

Retrieves information about the Square Catalog API, such as batch size
limits that can be used by the `BatchUpsertCatalogObjects` endpoint.*/
    pub fn catalog_info(&self) -> request::CatalogInfoRequest {
        request::CatalogInfoRequest {
            http_client: &self,
        }
    }
    /**ListCatalog

Returns a list of all [CatalogObject](entity:CatalogObject)s of the specified types in the catalog.

The `types` parameter is specified as a comma-separated list of the [CatalogObjectType](entity:CatalogObjectType) values,
for example, "`ITEM`, `ITEM_VARIATION`, `MODIFIER`, `MODIFIER_LIST`, `CATEGORY`, `DISCOUNT`, `TAX`, `IMAGE`".

__Important:__ ListCatalog does not return deleted catalog items. To retrieve
deleted catalog items, use [SearchCatalogObjects](api-endpoint:Catalog-SearchCatalogObjects)
and set the `include_deleted_objects` attribute value to `true`.*/
    pub fn list_catalog(&self) -> request::ListCatalogRequest {
        request::ListCatalogRequest {
            http_client: &self,
            catalog_version: None,
            cursor: None,
            types: None,
        }
    }
    /**UpsertCatalogObject

Creates or updates the target [CatalogObject](entity:CatalogObject).

See endpoint docs at <https://developer.squareup.com/docs/catalog-api/build-with-catalog>.*/
    pub fn upsert_catalog_object(
        &self,
        idempotency_key: &str,
        object: CatalogObject,
    ) -> request::UpsertCatalogObjectRequest {
        request::UpsertCatalogObjectRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            object,
        }
    }
    /**RetrieveCatalogObject

Returns a single [CatalogItem](entity:CatalogItem) as a
[CatalogObject](entity:CatalogObject) based on the provided ID. The returned
object includes all of the relevant [CatalogItem](entity:CatalogItem)
information including: [CatalogItemVariation](entity:CatalogItemVariation)
children, references to its
[CatalogModifierList](entity:CatalogModifierList) objects, and the ids of
any [CatalogTax](entity:CatalogTax) objects that apply to it.*/
    pub fn retrieve_catalog_object(
        &self,
        object_id: &str,
    ) -> request::RetrieveCatalogObjectRequest {
        request::RetrieveCatalogObjectRequest {
            http_client: &self,
            catalog_version: None,
            include_related_objects: None,
            object_id: object_id.to_owned(),
        }
    }
    /**DeleteCatalogObject

Deletes a single [CatalogObject](entity:CatalogObject) based on the
provided ID and returns the set of successfully deleted IDs in the response.
Deletion is a cascading event such that all children of the targeted object
are also deleted. For example, deleting a [CatalogItem](entity:CatalogItem)
will also delete all of its
[CatalogItemVariation](entity:CatalogItemVariation) children.*/
    pub fn delete_catalog_object(
        &self,
        object_id: &str,
    ) -> request::DeleteCatalogObjectRequest {
        request::DeleteCatalogObjectRequest {
            http_client: &self,
            object_id: object_id.to_owned(),
        }
    }
    /**SearchCatalogObjects

Searches for [CatalogObject](entity:CatalogObject) of any type by matching supported search attribute values,
excluding custom attribute values on items or item variations, against one or more of the specified query filters.

This (`SearchCatalogObjects`) endpoint differs from the [SearchCatalogItems](api-endpoint:Catalog-SearchCatalogItems)
endpoint in the following aspects:

- `SearchCatalogItems` can only search for items or item variations, whereas `SearchCatalogObjects` can search for any type of catalog objects.
- `SearchCatalogItems` supports the custom attribute query filters to return items or item variations that contain custom attribute values, where `SearchCatalogObjects` does not.
- `SearchCatalogItems` does not support the `include_deleted_objects` filter to search for deleted items or item variations, whereas `SearchCatalogObjects` does.
- The both endpoints have different call conventions, including the query filter formats.

See endpoint docs at <https://developer.squareup.com/docs/catalog-api/search-catalog-objects>.*/
    pub fn search_catalog_objects(&self) -> request::SearchCatalogObjectsRequest {
        request::SearchCatalogObjectsRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            include_deleted_objects: None,
            include_related_objects: None,
            limit: None,
            object_types: None,
            query: None,
        }
    }
    /**SearchCatalogItems

Searches for catalog items or item variations by matching supported search attribute values, including
custom attribute values, against one or more of the specified query filters.

This (`SearchCatalogItems`) endpoint differs from the [SearchCatalogObjects](api-endpoint:Catalog-SearchCatalogObjects)
endpoint in the following aspects:

- `SearchCatalogItems` can only search for items or item variations, whereas `SearchCatalogObjects` can search for any type of catalog objects.
- `SearchCatalogItems` supports the custom attribute query filters to return items or item variations that contain custom attribute values, where `SearchCatalogObjects` does not.
- `SearchCatalogItems` does not support the `include_deleted_objects` filter to search for deleted items or item variations, whereas `SearchCatalogObjects` does.
- The both endpoints use different call conventions, including the query filter formats.

See endpoint docs at <https://developer.squareup.com/docs/catalog-api/search-catalog-items>.*/
    pub fn search_catalog_items(&self) -> request::SearchCatalogItemsRequest {
        request::SearchCatalogItemsRequest {
            http_client: &self,
            category_ids: None,
            cursor: None,
            custom_attribute_filters: None,
            enabled_location_ids: None,
            limit: None,
            product_types: None,
            sort_order: None,
            stock_levels: None,
            text_filter: None,
        }
    }
    /**UpdateItemModifierLists

Updates the [CatalogModifierList](entity:CatalogModifierList) objects
that apply to the targeted [CatalogItem](entity:CatalogItem) without having
to perform an upsert on the entire item.*/
    pub fn update_item_modifier_lists(
        &self,
        item_ids: &[&str],
    ) -> request::UpdateItemModifierListsRequest {
        request::UpdateItemModifierListsRequest {
            http_client: &self,
            item_ids: item_ids.iter().map(|&x| x.to_owned()).collect(),
            modifier_lists_to_disable: None,
            modifier_lists_to_enable: None,
        }
    }
    /**UpdateItemTaxes

Updates the [CatalogTax](entity:CatalogTax) objects that apply to the
targeted [CatalogItem](entity:CatalogItem) without having to perform an
upsert on the entire item.*/
    pub fn update_item_taxes(
        &self,
        item_ids: &[&str],
    ) -> request::UpdateItemTaxesRequest {
        request::UpdateItemTaxesRequest {
            http_client: &self,
            item_ids: item_ids.iter().map(|&x| x.to_owned()).collect(),
            taxes_to_disable: None,
            taxes_to_enable: None,
        }
    }
    /**ListCustomers

Lists customer profiles associated with a Square account.

Under normal operating conditions, newly created or updated customer profiles become available
for the listing operation in well under 30 seconds. Occasionally, propagation of the new or updated
profiles can take closer to one minute or longer, especially during network incidents and outages.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/retrieve-profiles#retrieve-a-list-of-customer-profiles>.*/
    pub fn list_customers(&self) -> request::ListCustomersRequest {
        request::ListCustomersRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            sort_field: None,
            sort_order: None,
        }
    }
    /**CreateCustomer

Creates a new customer for a business.

You must provide at least one of the following values in your request to this
endpoint:

- `given_name`
- `family_name`
- `company_name`
- `email_address`
- `phone_number`

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#create-a-customer-profile>.*/
    pub fn create_customer(&self) -> request::CreateCustomerRequest {
        request::CreateCustomerRequest {
            http_client: &self,
            address: None,
            birthday: None,
            company_name: None,
            email_address: None,
            family_name: None,
            given_name: None,
            idempotency_key: None,
            nickname: None,
            note: None,
            phone_number: None,
            reference_id: None,
            tax_ids: None,
        }
    }
    /**ListCustomerCustomAttributeDefinitions

Lists the customer-related [custom attribute definitions](entity:CustomAttributeDefinition) that belong to a Square seller account.

When all response pages are retrieved, the results include all custom attribute definitions
that are visible to the requesting application, including those that are created by other
applications and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that
seller-defined custom attributes (also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attribute-definitions#list-customer-custom-attribute-definitions>.*/
    pub fn list_customer_custom_attribute_definitions(
        &self,
    ) -> request::ListCustomerCustomAttributeDefinitionsRequest {
        request::ListCustomerCustomAttributeDefinitionsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
        }
    }
    /**CreateCustomerCustomAttributeDefinition

Creates a customer-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account.
Use this endpoint to define a custom attribute that can be associated with customer profiles.

A custom attribute definition specifies the `key`, `visibility`, `schema`, and other properties
for a custom attribute. After the definition is created, you can call
[UpsertCustomerCustomAttribute](api-endpoint:CustomerCustomAttributes-UpsertCustomerCustomAttribute) or
[BulkUpsertCustomerCustomAttributes](api-endpoint:CustomerCustomAttributes-BulkUpsertCustomerCustomAttributes)
to set the custom attribute for customer profiles in the seller's Customer Directory.

Sellers can view all custom attributes in exported customer data, including those set to
`VISIBILITY_HIDDEN`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attribute-definitions#create-customer-custom-attribute-definition>.*/
    pub fn create_customer_custom_attribute_definition(
        &self,
        custom_attribute_definition: CustomAttributeDefinition,
    ) -> request::CreateCustomerCustomAttributeDefinitionRequest {
        request::CreateCustomerCustomAttributeDefinitionRequest {
            http_client: &self,
            custom_attribute_definition,
            idempotency_key: None,
        }
    }
    /**RetrieveCustomerCustomAttributeDefinition

Retrieves a customer-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.

To retrieve a custom attribute definition created by another application, the `visibility`
setting must be `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes
(also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attribute-definitions#retrieve-customer-custom-attribute-definition>.*/
    pub fn retrieve_customer_custom_attribute_definition(
        &self,
        key: &str,
    ) -> request::RetrieveCustomerCustomAttributeDefinitionRequest {
        request::RetrieveCustomerCustomAttributeDefinitionRequest {
            http_client: &self,
            key: key.to_owned(),
            version: None,
        }
    }
    /**UpdateCustomerCustomAttributeDefinition

Updates a customer-related [custom attribute definition](entity:CustomAttributeDefinition) for a Square seller account.

Use this endpoint to update the following fields: `name`, `description`, `visibility`, or the
`schema` for a `Selection` data type.

Only the definition owner can update a custom attribute definition. Note that sellers can view
all custom attributes in exported customer data, including those set to `VISIBILITY_HIDDEN`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attribute-definitions#update-customer-custom-attribute-definition>.*/
    pub fn update_customer_custom_attribute_definition(
        &self,
        custom_attribute_definition: CustomAttributeDefinition,
        key: &str,
    ) -> request::UpdateCustomerCustomAttributeDefinitionRequest {
        request::UpdateCustomerCustomAttributeDefinitionRequest {
            http_client: &self,
            custom_attribute_definition,
            idempotency_key: None,
            key: key.to_owned(),
        }
    }
    /**DeleteCustomerCustomAttributeDefinition

Deletes a customer-related [custom attribute definition](entity:CustomAttributeDefinition) from a Square seller account.

Deleting a custom attribute definition also deletes the corresponding custom attribute from
all customer profiles in the seller's Customer Directory.

Only the definition owner can delete a custom attribute definition.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attribute-definitions#delete-customer-custom-attribute-definition>.*/
    pub fn delete_customer_custom_attribute_definition(
        &self,
        key: &str,
    ) -> request::DeleteCustomerCustomAttributeDefinitionRequest {
        request::DeleteCustomerCustomAttributeDefinitionRequest {
            http_client: &self,
            key: key.to_owned(),
        }
    }
    /**BulkUpsertCustomerCustomAttributes

Creates or updates [custom attributes](entity:CustomAttribute) for customer profiles as a bulk operation.

Use this endpoint to set the value of one or more custom attributes for one or more customer profiles.
A custom attribute is based on a custom attribute definition in a Square seller account, which is
created using the [CreateCustomerCustomAttributeDefinition](api-endpoint:CustomerCustomAttributes-CreateCustomerCustomAttributeDefinition) endpoint.

This `BulkUpsertCustomerCustomAttributes` endpoint accepts a map of 1 to 25 individual upsert
requests and returns a map of individual upsert responses. Each upsert request has a unique ID
and provides a customer ID and custom attribute. Each upsert response is returned with the ID
of the corresponding request.

To create or update a custom attribute owned by another application, the `visibility` setting
must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes
(also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#bulk-upsert-customer-custom-attributes>.*/
    pub fn bulk_upsert_customer_custom_attributes(
        &self,
        values: serde_json::Value,
    ) -> request::BulkUpsertCustomerCustomAttributesRequest {
        request::BulkUpsertCustomerCustomAttributesRequest {
            http_client: &self,
            values,
        }
    }
    /**ListCustomerGroups

Retrieves the list of customer groups of a business.

See endpoint docs at <https://developer.squareup.com/docs/customer-groups-api/how-to-use-it#list-customer-groups>.*/
    pub fn list_customer_groups(&self) -> request::ListCustomerGroupsRequest {
        request::ListCustomerGroupsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
        }
    }
    /**CreateCustomerGroup

Creates a new customer group for a business.

The request must include the `name` value of the group.

See endpoint docs at <https://developer.squareup.com/docs/customer-groups-api/how-to-use-it#create-a-customer-group>.*/
    pub fn create_customer_group(
        &self,
        group: CustomerGroup,
    ) -> request::CreateCustomerGroupRequest {
        request::CreateCustomerGroupRequest {
            http_client: &self,
            group,
            idempotency_key: None,
        }
    }
    /**RetrieveCustomerGroup

Retrieves a specific customer group as identified by the `group_id` value.*/
    pub fn retrieve_customer_group(
        &self,
        group_id: &str,
    ) -> request::RetrieveCustomerGroupRequest {
        request::RetrieveCustomerGroupRequest {
            http_client: &self,
            group_id: group_id.to_owned(),
        }
    }
    /**UpdateCustomerGroup

Updates a customer group as identified by the `group_id` value.*/
    pub fn update_customer_group(
        &self,
        group: CustomerGroup,
        group_id: &str,
    ) -> request::UpdateCustomerGroupRequest {
        request::UpdateCustomerGroupRequest {
            http_client: &self,
            group,
            group_id: group_id.to_owned(),
        }
    }
    /**DeleteCustomerGroup

Deletes a customer group as identified by the `group_id` value.*/
    pub fn delete_customer_group(
        &self,
        group_id: &str,
    ) -> request::DeleteCustomerGroupRequest {
        request::DeleteCustomerGroupRequest {
            http_client: &self,
            group_id: group_id.to_owned(),
        }
    }
    /**SearchCustomers

Searches the customer profiles associated with a Square account using one or more supported query filters.

Calling `SearchCustomers` without any explicit query filter returns all
customer profiles ordered alphabetically based on `given_name` and
`family_name`.

Under normal operating conditions, newly created or updated customer profiles become available
for the search operation in well under 30 seconds. Occasionally, propagation of the new or updated
profiles can take closer to one minute or longer, especially during network incidents and outages.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/search-customers>.*/
    pub fn search_customers(&self) -> request::SearchCustomersRequest {
        request::SearchCustomersRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**ListCustomerSegments

Retrieves the list of customer segments of a business.

See endpoint docs at <https://developer.squareup.com/docs/customer-segments-api/how-to-use-it#list-customer-segments>.*/
    pub fn list_customer_segments(&self) -> request::ListCustomerSegmentsRequest {
        request::ListCustomerSegmentsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
        }
    }
    /**RetrieveCustomerSegment

Retrieves a specific customer segment as identified by the `segment_id` value.

See endpoint docs at <https://developer.squareup.com/docs/customer-segments-api/how-to-use-it#retrieve-segment-membership-information-for-a-customer>.*/
    pub fn retrieve_customer_segment(
        &self,
        segment_id: &str,
    ) -> request::RetrieveCustomerSegmentRequest {
        request::RetrieveCustomerSegmentRequest {
            http_client: &self,
            segment_id: segment_id.to_owned(),
        }
    }
    /**RetrieveCustomer

Returns details for a single customer.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/retrieve-profiles#retrieve-the-customer-profile-of-a-given-id>.*/
    pub fn retrieve_customer(
        &self,
        customer_id: &str,
    ) -> request::RetrieveCustomerRequest {
        request::RetrieveCustomerRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
        }
    }
    /**UpdateCustomer

Updates a customer profile. To change an attribute, specify the new value. To remove an attribute, specify the value as an empty string or empty object.

As a best practice, you should include the `version` field in the request to enable [optimistic concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency) control. The value must be set to the current version of the customer profile.

To update a customer profile that was created by merging existing profiles, you must use the ID of the newly created profile.

You cannot use this endpoint to change cards on file. To make changes, use the [Cards API](api:Cards) or [Gift Cards API](api:GiftCards).

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#update-a-customer-profile>.*/
    pub fn update_customer(&self, customer_id: &str) -> request::UpdateCustomerRequest {
        request::UpdateCustomerRequest {
            http_client: &self,
            address: None,
            birthday: None,
            company_name: None,
            customer_id: customer_id.to_owned(),
            email_address: None,
            family_name: None,
            given_name: None,
            nickname: None,
            note: None,
            phone_number: None,
            reference_id: None,
            tax_ids: None,
            version: None,
        }
    }
    /**DeleteCustomer

Deletes a customer profile from a business. This operation also unlinks any associated cards on file.

As a best practice, you should include the `version` field in the request to enable [optimistic concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency) control. The value must be set to the current version of the customer profile.

To delete a customer profile that was created by merging existing profiles, you must use the ID of the newly created profile.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/keep-records#delete-a-customer-profile>.*/
    pub fn delete_customer(&self, customer_id: &str) -> request::DeleteCustomerRequest {
        request::DeleteCustomerRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            version: None,
        }
    }
    /**CreateCustomerCard

Adds a card on file to an existing customer.

As with charges, calls to `CreateCustomerCard` are idempotent. Multiple
calls with the same card nonce return the same card record that was created
with the provided nonce during the _first_ call.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/integrate-with-other-services#save-cards-on-file>.*/
    pub fn create_customer_card(
        &self,
        card_nonce: &str,
        customer_id: &str,
    ) -> request::CreateCustomerCardRequest {
        request::CreateCustomerCardRequest {
            http_client: &self,
            billing_address: None,
            card_nonce: card_nonce.to_owned(),
            cardholder_name: None,
            customer_id: customer_id.to_owned(),
            verification_token: None,
        }
    }
    /**DeleteCustomerCard

Removes a card on file from a customer.

See endpoint docs at <https://developer.squareup.com/docs/customers-api/use-the-api/integrate-with-other-services#save-cards-on-file>.*/
    pub fn delete_customer_card(
        &self,
        card_id: &str,
        customer_id: &str,
    ) -> request::DeleteCustomerCardRequest {
        request::DeleteCustomerCardRequest {
            http_client: &self,
            card_id: card_id.to_owned(),
            customer_id: customer_id.to_owned(),
        }
    }
    /**ListCustomerCustomAttributes

Lists the [custom attributes](entity:CustomAttribute) associated with a customer profile.

You can use the `with_definitions` query parameter to also retrieve custom attribute definitions
in the same call.

When all response pages are retrieved, the results include all custom attributes that are
visible to the requesting application, including those that are owned by other applications
and set to `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#list-customer-custom-attributes>.*/
    pub fn list_customer_custom_attributes(
        &self,
        customer_id: &str,
    ) -> request::ListCustomerCustomAttributesRequest {
        request::ListCustomerCustomAttributesRequest {
            http_client: &self,
            cursor: None,
            customer_id: customer_id.to_owned(),
            limit: None,
            with_definitions: None,
        }
    }
    /**RetrieveCustomerCustomAttribute

Retrieves a [custom attribute](entity:CustomAttribute) associated with a customer profile.

You can use the `with_definition` query parameter to also retrieve the custom attribute definition
in the same call.

To retrieve a custom attribute owned by another application, the `visibility` setting must be
`VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes
(also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#retrieve-customer-custom-attribute>.*/
    pub fn retrieve_customer_custom_attribute(
        &self,
        customer_id: &str,
        key: &str,
    ) -> request::RetrieveCustomerCustomAttributeRequest {
        request::RetrieveCustomerCustomAttributeRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            key: key.to_owned(),
            version: None,
            with_definition: None,
        }
    }
    /**UpsertCustomerCustomAttribute

Creates or updates a [custom attribute](entity:CustomAttribute) for a customer profile.

Use this endpoint to set the value of a custom attribute for a specified customer profile.
A custom attribute is based on a custom attribute definition in a Square seller account, which
is created using the [CreateCustomerCustomAttributeDefinition](api-endpoint:CustomerCustomAttributes-CreateCustomerCustomAttributeDefinition) endpoint.

To create or update a custom attribute owned by another application, the `visibility` setting
must be `VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes
(also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#upsert-customer-custom-attribute>.*/
    pub fn upsert_customer_custom_attribute(
        &self,
        custom_attribute: CustomAttribute,
        customer_id: &str,
        key: &str,
    ) -> request::UpsertCustomerCustomAttributeRequest {
        request::UpsertCustomerCustomAttributeRequest {
            http_client: &self,
            custom_attribute,
            customer_id: customer_id.to_owned(),
            idempotency_key: None,
            key: key.to_owned(),
        }
    }
    /**DeleteCustomerCustomAttribute

Deletes a [custom attribute](entity:CustomAttribute) associated with a customer profile.

To delete a custom attribute owned by another application, the `visibility` setting must be
`VISIBILITY_READ_WRITE_VALUES`. Note that seller-defined custom attributes
(also known as custom fields) are always set to `VISIBILITY_READ_WRITE_VALUES`.

See endpoint docs at <https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#delete-a-customer-custom-attribute>.*/
    pub fn delete_customer_custom_attribute(
        &self,
        customer_id: &str,
        key: &str,
    ) -> request::DeleteCustomerCustomAttributeRequest {
        request::DeleteCustomerCustomAttributeRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            key: key.to_owned(),
        }
    }
    /**AddGroupToCustomer

Adds a group membership to a customer.

The customer is identified by the `customer_id` value
and the customer group is identified by the `group_id` value.

See endpoint docs at <https://developer.squareup.com/docs/customer-groups-api/how-to-use-it#add-customers-to-a-customer-group>.*/
    pub fn add_group_to_customer(
        &self,
        customer_id: &str,
        group_id: &str,
    ) -> request::AddGroupToCustomerRequest {
        request::AddGroupToCustomerRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            group_id: group_id.to_owned(),
        }
    }
    /**RemoveGroupFromCustomer

Removes a group membership from a customer.

The customer is identified by the `customer_id` value
and the customer group is identified by the `group_id` value.

See endpoint docs at <https://developer.squareup.com/docs/customer-groups-api/how-to-use-it#add-customers-to-a-customer-group>.*/
    pub fn remove_group_from_customer(
        &self,
        customer_id: &str,
        group_id: &str,
    ) -> request::RemoveGroupFromCustomerRequest {
        request::RemoveGroupFromCustomerRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            group_id: group_id.to_owned(),
        }
    }
    /**ListDeviceCodes

Lists all DeviceCodes associated with the merchant.*/
    pub fn list_device_codes(&self) -> request::ListDeviceCodesRequest {
        request::ListDeviceCodesRequest {
            http_client: &self,
            cursor: None,
            location_id: None,
            product_type: None,
            status: None,
        }
    }
    /**CreateDeviceCode

Creates a DeviceCode that can be used to login to a Square Terminal device to enter the connected
terminal mode.*/
    pub fn create_device_code(
        &self,
        device_code: DeviceCode,
        idempotency_key: &str,
    ) -> request::CreateDeviceCodeRequest {
        request::CreateDeviceCodeRequest {
            http_client: &self,
            device_code,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**GetDeviceCode

Retrieves DeviceCode with the associated ID.*/
    pub fn get_device_code(&self, id: &str) -> request::GetDeviceCodeRequest {
        request::GetDeviceCodeRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**ListDisputes

Returns a list of disputes associated with a particular account.

See endpoint docs at <https://developer.squareup.com/docs/disputes-api/process-disputes#access-dispute-information>.*/
    pub fn list_disputes(&self) -> request::ListDisputesRequest {
        request::ListDisputesRequest {
            http_client: &self,
            cursor: None,
            location_id: None,
            states: None,
        }
    }
    /**RetrieveDispute

Returns details about a specific dispute.*/
    pub fn retrieve_dispute(&self, dispute_id: &str) -> request::RetrieveDisputeRequest {
        request::RetrieveDisputeRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
        }
    }
    /**AcceptDispute

Accepts the loss on a dispute. Square returns the disputed amount to the cardholder and
updates the dispute state to ACCEPTED.

Square debits the disputed amount from the seller’s Square account. If the Square account
does not have sufficient funds, Square debits the associated bank account.

See endpoint docs at <https://developer.squareup.com/docs/disputes-api/process-disputes#accept-a-dispute>.*/
    pub fn accept_dispute(&self, dispute_id: &str) -> request::AcceptDisputeRequest {
        request::AcceptDisputeRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
        }
    }
    /**ListDisputeEvidence

Returns a list of evidence associated with a dispute.*/
    pub fn list_dispute_evidence(
        &self,
        dispute_id: &str,
    ) -> request::ListDisputeEvidenceRequest {
        request::ListDisputeEvidenceRequest {
            http_client: &self,
            cursor: None,
            dispute_id: dispute_id.to_owned(),
        }
    }
    /**CreateDisputeEvidenceFile

Uploads a file to use as evidence in a dispute challenge. The endpoint accepts HTTP
multipart/form-data file uploads in HEIC, HEIF, JPEG, PDF, PNG, and TIFF formats.

See endpoint docs at <https://developer.squareup.com/docs/disputes-api/process-disputes#challenge-a-dispute>.*/
    pub fn create_dispute_evidence_file(
        &self,
        dispute_id: &str,
    ) -> request::CreateDisputeEvidenceFileRequest {
        request::CreateDisputeEvidenceFileRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
        }
    }
    /**CreateDisputeEvidenceText

Uploads text to use as evidence for a dispute challenge.

See endpoint docs at <https://developer.squareup.com/docs/disputes-api/process-disputes#challenge-a-dispute>.*/
    pub fn create_dispute_evidence_text(
        &self,
        dispute_id: &str,
        evidence_text: &str,
        idempotency_key: &str,
    ) -> request::CreateDisputeEvidenceTextRequest {
        request::CreateDisputeEvidenceTextRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
            evidence_text: evidence_text.to_owned(),
            evidence_type: None,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**RetrieveDisputeEvidence

Returns the metadata for the evidence specified in the request URL path.

You must maintain a copy of any evidence uploaded if you want to reference it later. Evidence cannot be downloaded after you upload it.*/
    pub fn retrieve_dispute_evidence(
        &self,
        dispute_id: &str,
        evidence_id: &str,
    ) -> request::RetrieveDisputeEvidenceRequest {
        request::RetrieveDisputeEvidenceRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
            evidence_id: evidence_id.to_owned(),
        }
    }
    /**DeleteDisputeEvidence

Removes specified evidence from a dispute.
Square does not send the bank any evidence that is removed.*/
    pub fn delete_dispute_evidence(
        &self,
        dispute_id: &str,
        evidence_id: &str,
    ) -> request::DeleteDisputeEvidenceRequest {
        request::DeleteDisputeEvidenceRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
            evidence_id: evidence_id.to_owned(),
        }
    }
    /**SubmitEvidence

Submits evidence to the cardholder's bank.

The evidence submitted by this endpoint includes evidence uploaded
using the [CreateDisputeEvidenceFile](api-endpoint:Disputes-CreateDisputeEvidenceFile) and
[CreateDisputeEvidenceText](api-endpoint:Disputes-CreateDisputeEvidenceText) endpoints and
evidence automatically provided by Square, when available. Evidence cannot be removed from
a dispute after submission.

See endpoint docs at <https://developer.squareup.com/docs/disputes-api/process-disputes#challenge-a-dispute>.*/
    pub fn submit_evidence(&self, dispute_id: &str) -> request::SubmitEvidenceRequest {
        request::SubmitEvidenceRequest {
            http_client: &self,
            dispute_id: dispute_id.to_owned(),
        }
    }
    ///ListEmployees
    pub fn list_employees(&self) -> request::ListEmployeesRequest {
        request::ListEmployeesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: None,
            status: None,
        }
    }
    ///RetrieveEmployee
    pub fn retrieve_employee(&self, id: &str) -> request::RetrieveEmployeeRequest {
        request::RetrieveEmployeeRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**ListGiftCards

Lists all gift cards. You can specify optional filters to retrieve
a subset of the gift cards. Results are sorted by `created_at` in ascending order.*/
    pub fn list_gift_cards(&self) -> request::ListGiftCardsRequest {
        request::ListGiftCardsRequest {
            http_client: &self,
            cursor: None,
            customer_id: None,
            limit: None,
            state: None,
            type_: None,
        }
    }
    /**CreateGiftCard

Creates a digital gift card or registers a physical (plastic) gift card. After the gift card
is created, you must call [CreateGiftCardActivity](api-endpoint:GiftCardActivities-CreateGiftCardActivity)
to activate the card with an initial balance before it can be used for payment.

See endpoint docs at <https://developer.squareup.com/docs/gift-cards/using-gift-cards-api#selling-square-gift-cards>.*/
    pub fn create_gift_card(
        &self,
        gift_card: GiftCard,
        idempotency_key: &str,
        location_id: &str,
    ) -> request::CreateGiftCardRequest {
        request::CreateGiftCardRequest {
            http_client: &self,
            gift_card,
            idempotency_key: idempotency_key.to_owned(),
            location_id: location_id.to_owned(),
        }
    }
    /**ListGiftCardActivities

Lists gift card activities. By default, you get gift card activities for all
gift cards in the seller's account. You can optionally specify query parameters to
filter the list. For example, you can get a list of gift card activities for a gift card,
for all gift cards in a specific region, or for activities within a time window.*/
    pub fn list_gift_card_activities(&self) -> request::ListGiftCardActivitiesRequest {
        request::ListGiftCardActivitiesRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            end_time: None,
            gift_card_id: None,
            limit: None,
            location_id: None,
            sort_order: None,
            type_: None,
        }
    }
    /**CreateGiftCardActivity

Creates a gift card activity to manage the balance or state of a [gift card](entity:GiftCard).
For example, you create an `ACTIVATE` activity to activate a gift card with an initial balance
before the gift card can be used.

See endpoint docs at <https://developer.squareup.com/docs/gift-cards/using-gift-cards-api#managing-the-gift-card-balance-or-state>.*/
    pub fn create_gift_card_activity(
        &self,
        gift_card_activity: GiftCardActivity,
        idempotency_key: &str,
    ) -> request::CreateGiftCardActivityRequest {
        request::CreateGiftCardActivityRequest {
            http_client: &self,
            gift_card_activity,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**RetrieveGiftCardFromGAN

Retrieves a gift card using the gift card account number (GAN).*/
    pub fn retrieve_gift_card_from_gan(
        &self,
        gan: &str,
    ) -> request::RetrieveGiftCardFromGanRequest {
        request::RetrieveGiftCardFromGanRequest {
            http_client: &self,
            gan: gan.to_owned(),
        }
    }
    /**RetrieveGiftCardFromNonce

Retrieves a gift card using a secure payment token that represents the gift card.*/
    pub fn retrieve_gift_card_from_nonce(
        &self,
        nonce: &str,
    ) -> request::RetrieveGiftCardFromNonceRequest {
        request::RetrieveGiftCardFromNonceRequest {
            http_client: &self,
            nonce: nonce.to_owned(),
        }
    }
    /**LinkCustomerToGiftCard

Links a customer to a gift card, which is also referred to as adding a card on file.

See endpoint docs at <https://developer.squareup.com/docs/gift-cards/additional-considerations#manage-gift-cards-on-file>.*/
    pub fn link_customer_to_gift_card(
        &self,
        customer_id: &str,
        gift_card_id: &str,
    ) -> request::LinkCustomerToGiftCardRequest {
        request::LinkCustomerToGiftCardRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            gift_card_id: gift_card_id.to_owned(),
        }
    }
    /**UnlinkCustomerFromGiftCard

Unlinks a customer from a gift card, which is also referred to as removing a card on file.

See endpoint docs at <https://developer.squareup.com/docs/gift-cards/additional-considerations#manage-gift-cards-on-file>.*/
    pub fn unlink_customer_from_gift_card(
        &self,
        customer_id: &str,
        gift_card_id: &str,
    ) -> request::UnlinkCustomerFromGiftCardRequest {
        request::UnlinkCustomerFromGiftCardRequest {
            http_client: &self,
            customer_id: customer_id.to_owned(),
            gift_card_id: gift_card_id.to_owned(),
        }
    }
    /**RetrieveGiftCard

Retrieves a gift card using the gift card ID.*/
    pub fn retrieve_gift_card(&self, id: &str) -> request::RetrieveGiftCardRequest {
        request::RetrieveGiftCardRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**DeprecatedRetrieveInventoryAdjustment

Deprecated version of [RetrieveInventoryAdjustment](api-endpoint:Inventory-RetrieveInventoryAdjustment) after the endpoint URL
is updated to conform to the standard convention.*/
    pub fn deprecated_retrieve_inventory_adjustment(
        &self,
        adjustment_id: &str,
    ) -> request::DeprecatedRetrieveInventoryAdjustmentRequest {
        request::DeprecatedRetrieveInventoryAdjustmentRequest {
            http_client: &self,
            adjustment_id: adjustment_id.to_owned(),
        }
    }
    /**RetrieveInventoryAdjustment

Returns the [InventoryAdjustment](entity:InventoryAdjustment) object
with the provided `adjustment_id`.*/
    pub fn retrieve_inventory_adjustment(
        &self,
        adjustment_id: &str,
    ) -> request::RetrieveInventoryAdjustmentRequest {
        request::RetrieveInventoryAdjustmentRequest {
            http_client: &self,
            adjustment_id: adjustment_id.to_owned(),
        }
    }
    /**DeprecatedBatchChangeInventory

Deprecated version of [BatchChangeInventory](api-endpoint:Inventory-BatchChangeInventory) after the endpoint URL
is updated to conform to the standard convention.

See endpoint docs at <https://developer.squareup.com/docs/inventory-api/cookbook/reconcile-computed-quantity>.*/
    pub fn deprecated_batch_change_inventory(
        &self,
        idempotency_key: &str,
    ) -> request::DeprecatedBatchChangeInventoryRequest {
        request::DeprecatedBatchChangeInventoryRequest {
            http_client: &self,
            changes: None,
            idempotency_key: idempotency_key.to_owned(),
            ignore_unchanged_counts: None,
        }
    }
    /**DeprecatedBatchRetrieveInventoryChanges

Deprecated version of [BatchRetrieveInventoryChanges](api-endpoint:Inventory-BatchRetrieveInventoryChanges) after the endpoint URL
is updated to conform to the standard convention.*/
    pub fn deprecated_batch_retrieve_inventory_changes(
        &self,
    ) -> request::DeprecatedBatchRetrieveInventoryChangesRequest {
        request::DeprecatedBatchRetrieveInventoryChangesRequest {
            http_client: &self,
            catalog_object_ids: None,
            cursor: None,
            limit: None,
            location_ids: None,
            states: None,
            statuses: None,
            types: None,
            updated_after: None,
            updated_before: None,
        }
    }
    /**DeprecatedBatchRetrieveInventoryCounts

Deprecated version of [BatchRetrieveInventoryCounts](api-endpoint:Inventory-BatchRetrieveInventoryCounts) after the endpoint URL
is updated to conform to the standard convention.*/
    pub fn deprecated_batch_retrieve_inventory_counts(
        &self,
    ) -> request::DeprecatedBatchRetrieveInventoryCountsRequest {
        request::DeprecatedBatchRetrieveInventoryCountsRequest {
            http_client: &self,
            catalog_object_ids: None,
            cursor: None,
            limit: None,
            location_ids: None,
            states: None,
            updated_after: None,
        }
    }
    /**BatchChangeInventory

Applies adjustments and counts to the provided item quantities.

On success: returns the current calculated counts for all objects
referenced in the request.
On failure: returns a list of related errors.

See endpoint docs at <https://developer.squareup.com/docs/inventory-api/cookbook/reconcile-computed-quantity>.*/
    pub fn batch_change_inventory(
        &self,
        idempotency_key: &str,
    ) -> request::BatchChangeInventoryRequest {
        request::BatchChangeInventoryRequest {
            http_client: &self,
            changes: None,
            idempotency_key: idempotency_key.to_owned(),
            ignore_unchanged_counts: None,
        }
    }
    /**BatchRetrieveInventoryChanges

Returns historical physical counts and adjustments based on the
provided filter criteria.

Results are paginated and sorted in ascending order according their
`occurred_at` timestamp (oldest first).

BatchRetrieveInventoryChanges is a catch-all query endpoint for queries
that cannot be handled by other, simpler endpoints.*/
    pub fn batch_retrieve_inventory_changes(
        &self,
    ) -> request::BatchRetrieveInventoryChangesRequest {
        request::BatchRetrieveInventoryChangesRequest {
            http_client: &self,
            catalog_object_ids: None,
            cursor: None,
            limit: None,
            location_ids: None,
            states: None,
            statuses: None,
            types: None,
            updated_after: None,
            updated_before: None,
        }
    }
    /**BatchRetrieveInventoryCounts

Returns current counts for the provided
[CatalogObject](entity:CatalogObject)s at the requested
[Location](entity:Location)s.

Results are paginated and sorted in descending order according to their
`calculated_at` timestamp (newest first).

When `updated_after` is specified, only counts that have changed since that
time (based on the server timestamp for the most recent change) are
returned. This allows clients to perform a "sync" operation, for example
in response to receiving a Webhook notification.*/
    pub fn batch_retrieve_inventory_counts(
        &self,
    ) -> request::BatchRetrieveInventoryCountsRequest {
        request::BatchRetrieveInventoryCountsRequest {
            http_client: &self,
            catalog_object_ids: None,
            cursor: None,
            limit: None,
            location_ids: None,
            states: None,
            updated_after: None,
        }
    }
    /**DeprecatedRetrieveInventoryPhysicalCount

Deprecated version of [RetrieveInventoryPhysicalCount](api-endpoint:Inventory-RetrieveInventoryPhysicalCount) after the endpoint URL
is updated to conform to the standard convention.*/
    pub fn deprecated_retrieve_inventory_physical_count(
        &self,
        physical_count_id: &str,
    ) -> request::DeprecatedRetrieveInventoryPhysicalCountRequest {
        request::DeprecatedRetrieveInventoryPhysicalCountRequest {
            http_client: &self,
            physical_count_id: physical_count_id.to_owned(),
        }
    }
    /**RetrieveInventoryPhysicalCount

Returns the [InventoryPhysicalCount](entity:InventoryPhysicalCount)
object with the provided `physical_count_id`.*/
    pub fn retrieve_inventory_physical_count(
        &self,
        physical_count_id: &str,
    ) -> request::RetrieveInventoryPhysicalCountRequest {
        request::RetrieveInventoryPhysicalCountRequest {
            http_client: &self,
            physical_count_id: physical_count_id.to_owned(),
        }
    }
    /**RetrieveInventoryTransfer

Returns the [InventoryTransfer](entity:InventoryTransfer) object
with the provided `transfer_id`.*/
    pub fn retrieve_inventory_transfer(
        &self,
        transfer_id: &str,
    ) -> request::RetrieveInventoryTransferRequest {
        request::RetrieveInventoryTransferRequest {
            http_client: &self,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**RetrieveInventoryCount

Retrieves the current calculated stock count for a given
[CatalogObject](entity:CatalogObject) at a given set of
[Location](entity:Location)s. Responses are paginated and unsorted.
For more sophisticated queries, use a batch endpoint.

See endpoint docs at <https://developer.squareup.com/docs/inventory-api/cookbook/retrieve-specific-instock-quantity>.*/
    pub fn retrieve_inventory_count(
        &self,
        catalog_object_id: &str,
    ) -> request::RetrieveInventoryCountRequest {
        request::RetrieveInventoryCountRequest {
            http_client: &self,
            catalog_object_id: catalog_object_id.to_owned(),
            cursor: None,
            location_ids: None,
        }
    }
    /**RetrieveInventoryChanges

Returns a set of physical counts and inventory adjustments for the
provided [CatalogObject](entity:CatalogObject) at the requested
[Location](entity:Location)s.

You can achieve the same result by calling [BatchRetrieveInventoryChanges](api-endpoint:Inventory-BatchRetrieveInventoryChanges)
and having the `catalog_object_ids` list contain a single element of the `CatalogObject` ID.

Results are paginated and sorted in descending order according to their
`occurred_at` timestamp (newest first).

There are no limits on how far back the caller can page. This endpoint can be
used to display recent changes for a specific item. For more
sophisticated queries, use a batch endpoint.

See endpoint docs at <https://developer.squareup.com/docs/inventory-api/cookbook/inventory-change-history>.*/
    pub fn retrieve_inventory_changes(
        &self,
        catalog_object_id: &str,
    ) -> request::RetrieveInventoryChangesRequest {
        request::RetrieveInventoryChangesRequest {
            http_client: &self,
            catalog_object_id: catalog_object_id.to_owned(),
            cursor: None,
            location_ids: None,
        }
    }
    /**ListInvoices

Returns a list of invoices for a given location. The response
is paginated. If truncated, the response includes a `cursor` that you
use in a subsequent request to retrieve the next set of invoices.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices>.*/
    pub fn list_invoices(&self, location_id: &str) -> request::ListInvoicesRequest {
        request::ListInvoicesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: location_id.to_owned(),
        }
    }
    /**CreateInvoice

Creates a draft [invoice](entity:Invoice)
for an order created using the Orders API.

A draft invoice remains in your account and no action is taken.
You must publish the invoice before Square can process it (send it to the customer's email address or charge the customer’s card on file).

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/create-publish-invoices>.*/
    pub fn create_invoice(&self, invoice: Invoice) -> request::CreateInvoiceRequest {
        request::CreateInvoiceRequest {
            http_client: &self,
            idempotency_key: None,
            invoice,
        }
    }
    /**SearchInvoices

Searches for invoices from a location specified in
the filter. You can optionally specify customers in the filter for whom to
retrieve invoices. In the current implementation, you can only specify one location and
optionally one customer.

The response is paginated. If truncated, the response includes a `cursor`
that you use in a subsequent request to retrieve the next set of invoices.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices>.*/
    pub fn search_invoices(
        &self,
        query: InvoiceQuery,
    ) -> request::SearchInvoicesRequest {
        request::SearchInvoicesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query,
        }
    }
    /**GetInvoice

Retrieves an invoice by invoice ID.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices>.*/
    pub fn get_invoice(&self, invoice_id: &str) -> request::GetInvoiceRequest {
        request::GetInvoiceRequest {
            http_client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**UpdateInvoice

Updates an invoice by modifying fields, clearing fields, or both. For most updates, you can use a sparse
`Invoice` object to add fields or change values and use the `fields_to_clear` field to specify fields to clear.
However, some restrictions apply. For example, you cannot change the `order_id` or `location_id` field and you
must provide the complete `custom_fields` list to update a custom field. Published invoices have additional restrictions.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/update-invoices>.*/
    pub fn update_invoice(
        &self,
        invoice: Invoice,
        invoice_id: &str,
    ) -> request::UpdateInvoiceRequest {
        request::UpdateInvoiceRequest {
            http_client: &self,
            fields_to_clear: None,
            idempotency_key: None,
            invoice,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**DeleteInvoice

Deletes the specified invoice. When an invoice is deleted, the
associated order status changes to CANCELED. You can only delete a draft
invoice (you cannot delete a published invoice, including one that is scheduled for processing).

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/cancel-delete-invoices>.*/
    pub fn delete_invoice(&self, invoice_id: &str) -> request::DeleteInvoiceRequest {
        request::DeleteInvoiceRequest {
            http_client: &self,
            invoice_id: invoice_id.to_owned(),
            version: None,
        }
    }
    /**CancelInvoice

Cancels an invoice. The seller cannot collect payments for
the canceled invoice.

You cannot cancel an invoice in the `DRAFT` state or in a terminal state: `PAID`, `REFUNDED`, `CANCELED`, or `FAILED`.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/cancel-delete-invoices>.*/
    pub fn cancel_invoice(
        &self,
        invoice_id: &str,
        version: i64,
    ) -> request::CancelInvoiceRequest {
        request::CancelInvoiceRequest {
            http_client: &self,
            invoice_id: invoice_id.to_owned(),
            version,
        }
    }
    /**PublishInvoice

Publishes the specified draft invoice.

After an invoice is published, Square
follows up based on the invoice configuration. For example, Square
sends the invoice to the customer's email address, charges the customer's card on file, or does
nothing. Square also makes the invoice available on a Square-hosted invoice page.

The invoice `status` also changes from `DRAFT` to a status
based on the invoice configuration. For example, the status changes to `UNPAID` if
Square emails the invoice or `PARTIALLY_PAID` if Square charge a card on file for a portion of the
invoice amount.

See endpoint docs at <https://developer.squareup.com/docs/invoices-api/create-publish-invoices>.*/
    pub fn publish_invoice(
        &self,
        invoice_id: &str,
        version: i64,
    ) -> request::PublishInvoiceRequest {
        request::PublishInvoiceRequest {
            http_client: &self,
            idempotency_key: None,
            invoice_id: invoice_id.to_owned(),
            version,
        }
    }
    /**ListBreakTypes

Returns a paginated list of `BreakType` instances for a business.*/
    pub fn list_break_types(&self) -> request::ListBreakTypesRequest {
        request::ListBreakTypesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_id: None,
        }
    }
    /**CreateBreakType

Creates a new `BreakType`.

A `BreakType` is a template for creating `Break` objects.
You must provide the following values in your request to this
endpoint:

- `location_id`
- `break_name`
- `expected_duration`
- `is_paid`

You can only have three `BreakType` instances per location. If you attempt to add a fourth
`BreakType` for a location, an `INVALID_REQUEST_ERROR` "Exceeded limit of 3 breaks per location."
is returned.*/
    pub fn create_break_type(
        &self,
        break_type: BreakType,
    ) -> request::CreateBreakTypeRequest {
        request::CreateBreakTypeRequest {
            http_client: &self,
            break_type,
            idempotency_key: None,
        }
    }
    /**GetBreakType

Returns a single `BreakType` specified by `id`.*/
    pub fn get_break_type(&self, id: &str) -> request::GetBreakTypeRequest {
        request::GetBreakTypeRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**UpdateBreakType

Updates an existing `BreakType`.*/
    pub fn update_break_type(
        &self,
        break_type: BreakType,
        id: &str,
    ) -> request::UpdateBreakTypeRequest {
        request::UpdateBreakTypeRequest {
            http_client: &self,
            break_type,
            id: id.to_owned(),
        }
    }
    /**DeleteBreakType

Deletes an existing `BreakType`.

A `BreakType` can be deleted even if it is referenced from a `Shift`.*/
    pub fn delete_break_type(&self, id: &str) -> request::DeleteBreakTypeRequest {
        request::DeleteBreakTypeRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**ListEmployeeWages

Returns a paginated list of `EmployeeWage` instances for a business.*/
    pub fn list_employee_wages(&self) -> request::ListEmployeeWagesRequest {
        request::ListEmployeeWagesRequest {
            http_client: &self,
            cursor: None,
            employee_id: None,
            limit: None,
        }
    }
    /**GetEmployeeWage

Returns a single `EmployeeWage` specified by `id`.*/
    pub fn get_employee_wage(&self, id: &str) -> request::GetEmployeeWageRequest {
        request::GetEmployeeWageRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**CreateShift

Creates a new `Shift`.

A `Shift` represents a complete workday for a single employee.
You must provide the following values in your request to this
endpoint:

- `location_id`
- `employee_id`
- `start_at`

An attempt to create a new `Shift` can result in a `BAD_REQUEST` error when:
- The `status` of the new `Shift` is `OPEN` and the employee has another
shift with an `OPEN` status.
- The `start_at` date is in the future.
- The `start_at` or `end_at` date overlaps another shift for the same employee.
- The `Break` instances are set in the request and a break `start_at`
is before the `Shift.start_at`, a break `end_at` is after
the `Shift.end_at`, or both.*/
    pub fn create_shift(&self, shift: Shift) -> request::CreateShiftRequest {
        request::CreateShiftRequest {
            http_client: &self,
            idempotency_key: None,
            shift,
        }
    }
    /**SearchShifts

Returns a paginated list of `Shift` records for a business.
The list to be returned can be filtered by:
- Location IDs.
- Employee IDs.
- Shift status (`OPEN` and `CLOSED`).
- Shift start.
- Shift end.
- Workday details.

The list can be sorted by:
- `start_at`.
- `end_at`.
- `created_at`.
- `updated_at`.*/
    pub fn search_shifts(&self) -> request::SearchShiftsRequest {
        request::SearchShiftsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**GetShift

Returns a single `Shift` specified by `id`.*/
    pub fn get_shift(&self, id: &str) -> request::GetShiftRequest {
        request::GetShiftRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**UpdateShift

Updates an existing `Shift`.

When adding a `Break` to a `Shift`, any earlier `Break` instances in the `Shift` have
the `end_at` property set to a valid RFC-3339 datetime string.

When closing a `Shift`, all `Break` instances in the `Shift` must be complete with `end_at`
set on each `Break`.*/
    pub fn update_shift(&self, id: &str, shift: Shift) -> request::UpdateShiftRequest {
        request::UpdateShiftRequest {
            http_client: &self,
            id: id.to_owned(),
            shift,
        }
    }
    /**DeleteShift

Deletes a `Shift`.*/
    pub fn delete_shift(&self, id: &str) -> request::DeleteShiftRequest {
        request::DeleteShiftRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**ListTeamMemberWages

Returns a paginated list of `TeamMemberWage` instances for a business.*/
    pub fn list_team_member_wages(&self) -> request::ListTeamMemberWagesRequest {
        request::ListTeamMemberWagesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            team_member_id: None,
        }
    }
    /**GetTeamMemberWage

Returns a single `TeamMemberWage` specified by `id `.*/
    pub fn get_team_member_wage(&self, id: &str) -> request::GetTeamMemberWageRequest {
        request::GetTeamMemberWageRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**ListWorkweekConfigs

Returns a list of `WorkweekConfig` instances for a business.*/
    pub fn list_workweek_configs(&self) -> request::ListWorkweekConfigsRequest {
        request::ListWorkweekConfigsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
        }
    }
    /**UpdateWorkweekConfig

Updates a `WorkweekConfig`.*/
    pub fn update_workweek_config(
        &self,
        id: &str,
        workweek_config: WorkweekConfig,
    ) -> request::UpdateWorkweekConfigRequest {
        request::UpdateWorkweekConfigRequest {
            http_client: &self,
            id: id.to_owned(),
            workweek_config,
        }
    }
    /**ListLocations

Provides details about all of the seller's [locations](https://developer.squareup.com/docs/locations-api),
including those with an inactive status.

See endpoint docs at <https://developer.squareup.com/docs/locations-api#listlocations>.*/
    pub fn list_locations(&self) -> request::ListLocationsRequest {
        request::ListLocationsRequest {
            http_client: &self,
        }
    }
    /**CreateLocation

Creates a [location](https://developer.squareup.com/docs/locations-api).
Creating new locations allows for separate configuration of receipt layouts, item prices,
and sales reports. Developers can use locations to separate sales activity through applications
that integrate with Square from sales activity elsewhere in a seller's account.
Locations created programmatically with the Locations API last forever and
are visible to the seller for their own management. Therefore, ensure that
each location has a sensible and unique name.

See endpoint docs at <https://developer.squareup.com/docs/locations-api#createalocation>.*/
    pub fn create_location(&self) -> request::CreateLocationRequest {
        request::CreateLocationRequest {
            http_client: &self,
            location: None,
        }
    }
    /**RetrieveLocation

Retrieves details of a single location. Specify "main"
as the location ID to retrieve details of the [main location](https://developer.squareup.com/docs/locations-api#about-the-main-location).

See endpoint docs at <https://developer.squareup.com/docs/locations-api#retrievelocation>.*/
    pub fn retrieve_location(
        &self,
        location_id: &str,
    ) -> request::RetrieveLocationRequest {
        request::RetrieveLocationRequest {
            http_client: &self,
            location_id: location_id.to_owned(),
        }
    }
    /**UpdateLocation

Updates a [location](https://developer.squareup.com/docs/locations-api).

See endpoint docs at <https://developer.squareup.com/docs/locations-api#updatealocation>.*/
    pub fn update_location(&self, location_id: &str) -> request::UpdateLocationRequest {
        request::UpdateLocationRequest {
            http_client: &self,
            location: None,
            location_id: location_id.to_owned(),
        }
    }
    /**CreateCheckout

Links a `checkoutId` to a `checkout_page_url` that customers are
directed to in order to provide their payment information using a
payment processing workflow hosted on connect.squareup.com.


NOTE: The Checkout API has been updated with new features.
For more information, see [Checkout API highlights](https://developer.squareup.com/docs/checkout-api#checkout-api-highlights).
We recommend that you use the new [CreatePaymentLink](api-endpoint:Checkout-CreatePaymentLink) 
endpoint in place of this previously released endpoint.*/
    pub fn create_checkout(
        &self,
        idempotency_key: &str,
        location_id: &str,
        order: CreateOrderRequest,
    ) -> request::CreateCheckoutRequest {
        request::CreateCheckoutRequest {
            http_client: &self,
            additional_recipients: None,
            ask_for_shipping_address: None,
            idempotency_key: idempotency_key.to_owned(),
            location_id: location_id.to_owned(),
            merchant_support_email: None,
            note: None,
            order,
            pre_populate_buyer_email: None,
            pre_populate_shipping_address: None,
            redirect_url: None,
        }
    }
    /**ListTransactions

Lists transactions for a particular location.

Transactions include payment information from sales and exchanges and refund
information from returns and exchanges.

Max results per [page](https://developer.squareup.com/docs/working-with-apis/pagination): 50*/
    pub fn list_transactions(
        &self,
        location_id: &str,
    ) -> request::ListTransactionsRequest {
        request::ListTransactionsRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            end_time: None,
            location_id: location_id.to_owned(),
            sort_order: None,
        }
    }
    /**RetrieveTransaction

Retrieves details for a single transaction.*/
    pub fn retrieve_transaction(
        &self,
        location_id: &str,
        transaction_id: &str,
    ) -> request::RetrieveTransactionRequest {
        request::RetrieveTransactionRequest {
            http_client: &self,
            location_id: location_id.to_owned(),
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**CaptureTransaction

Captures a transaction that was created with the [Charge](api-endpoint:Transactions-Charge)
endpoint with a `delay_capture` value of `true`.


See [Delayed capture transactions](https://developer.squareup.com/docs/payments/transactions/overview#delayed-capture)
for more information.*/
    pub fn capture_transaction(
        &self,
        location_id: &str,
        transaction_id: &str,
    ) -> request::CaptureTransactionRequest {
        request::CaptureTransactionRequest {
            http_client: &self,
            location_id: location_id.to_owned(),
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**VoidTransaction

Cancels a transaction that was created with the [Charge](api-endpoint:Transactions-Charge)
endpoint with a `delay_capture` value of `true`.


See [Delayed capture transactions](https://developer.squareup.com/docs/payments/transactions/overview#delayed-capture)
for more information.*/
    pub fn void_transaction(
        &self,
        location_id: &str,
        transaction_id: &str,
    ) -> request::VoidTransactionRequest {
        request::VoidTransactionRequest {
            http_client: &self,
            location_id: location_id.to_owned(),
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**CreateLoyaltyAccount

Creates a loyalty account. To create a loyalty account, you must provide the `program_id` and a `mapping` with the `phone_number` of the buyer.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-accounts#create-loyalty-account>.*/
    pub fn create_loyalty_account(
        &self,
        idempotency_key: &str,
        loyalty_account: LoyaltyAccount,
    ) -> request::CreateLoyaltyAccountRequest {
        request::CreateLoyaltyAccountRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            loyalty_account,
        }
    }
    /**SearchLoyaltyAccounts

Searches for loyalty accounts in a loyalty program.

You can search for a loyalty account using the phone number or customer ID associated with the account. To return all loyalty accounts, specify an empty `query` object or omit it entirely.

Search results are sorted by `created_at` in ascending order.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-accounts#search-loyalty-accounts>.*/
    pub fn search_loyalty_accounts(&self) -> request::SearchLoyaltyAccountsRequest {
        request::SearchLoyaltyAccountsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**RetrieveLoyaltyAccount

Retrieves a loyalty account.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-accounts#retrieve-loyalty-account>.*/
    pub fn retrieve_loyalty_account(
        &self,
        account_id: &str,
    ) -> request::RetrieveLoyaltyAccountRequest {
        request::RetrieveLoyaltyAccountRequest {
            http_client: &self,
            account_id: account_id.to_owned(),
        }
    }
    /**AccumulateLoyaltyPoints

Adds points earned from a purchase to a [loyalty account](entity:LoyaltyAccount).

- If you are using the Orders API to manage orders, provide the `order_id`. Square reads the order
to compute the points earned from both the base loyalty program and an associated
[loyalty promotion](entity:LoyaltyPromotion). For purchases that qualify for multiple accrual
rules, Square computes points based on the accrual rule that grants the most points.
For purchases that qualify for multiple promotions, Square computes points based on the most
recently created promotion. A purchase must first qualify for program points to be eligible for promotion points.

- If you are not using the Orders API to manage orders, provide `points` with the number of points to add.
You must first perform a client-side computation of the points earned from the loyalty program and
loyalty promotion. For spend-based and visit-based programs, you can call [CalculateLoyaltyPoints](api-endpoint:Loyalty-CalculateLoyaltyPoints)
to compute the points earned from the loyalty program (but not points earned from a loyalty promotion).

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-points#accumulate-loyalty-points>.*/
    pub fn accumulate_loyalty_points(
        &self,
        args: request::AccumulateLoyaltyPointsRequired,
    ) -> request::AccumulateLoyaltyPointsRequest {
        request::AccumulateLoyaltyPointsRequest {
            http_client: &self,
            account_id: args.account_id.to_owned(),
            accumulate_points: args.accumulate_points,
            idempotency_key: args.idempotency_key.to_owned(),
            location_id: args.location_id.to_owned(),
        }
    }
    /**AdjustLoyaltyPoints

Adds points to or subtracts points from a buyer's account.

Use this endpoint only when you need to manually adjust points. Otherwise, in your application flow, you call
[AccumulateLoyaltyPoints](api-endpoint:Loyalty-AccumulateLoyaltyPoints)
to add points when a buyer pays for the purchase.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-points#adjust-loyalty-points>.*/
    pub fn adjust_loyalty_points(
        &self,
        account_id: &str,
        adjust_points: LoyaltyEventAdjustPoints,
        idempotency_key: &str,
    ) -> request::AdjustLoyaltyPointsRequest {
        request::AdjustLoyaltyPointsRequest {
            http_client: &self,
            account_id: account_id.to_owned(),
            adjust_points,
            allow_negative_balance: None,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**SearchLoyaltyEvents

Searches for loyalty events.

A Square loyalty program maintains a ledger of events that occur during the lifetime of a
buyer's loyalty account. Each change in the point balance
(for example, points earned, points redeemed, and points expired) is
recorded in the ledger. Using this endpoint, you can search the ledger for events.

Search results are sorted by `created_at` in descending order.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-events#search-loyalty-events>.*/
    pub fn search_loyalty_events(&self) -> request::SearchLoyaltyEventsRequest {
        request::SearchLoyaltyEventsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**ListLoyaltyPrograms

Returns a list of loyalty programs in the seller's account.
Loyalty programs define how buyers can earn points and redeem points for rewards. Square sellers can have only one loyalty program, which is created and managed from the Seller Dashboard. For more information, see [Loyalty Program Overview](https://developer.squareup.com/docs/loyalty/overview).


Replaced with [RetrieveLoyaltyProgram](api-endpoint:Loyalty-RetrieveLoyaltyProgram) when used with the keyword `main`.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-programs#list-loyalty-programs>.*/
    pub fn list_loyalty_programs(&self) -> request::ListLoyaltyProgramsRequest {
        request::ListLoyaltyProgramsRequest {
            http_client: &self,
        }
    }
    /**RetrieveLoyaltyProgram

Retrieves the loyalty program in a seller's account, specified by the program ID or the keyword `main`.

Loyalty programs define how buyers can earn points and redeem points for rewards. Square sellers can have only one loyalty program, which is created and managed from the Seller Dashboard. For more information, see [Loyalty Program Overview](https://developer.squareup.com/docs/loyalty/overview).

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-programs#retrieve-loyalty-program>.*/
    pub fn retrieve_loyalty_program(
        &self,
        program_id: &str,
    ) -> request::RetrieveLoyaltyProgramRequest {
        request::RetrieveLoyaltyProgramRequest {
            http_client: &self,
            program_id: program_id.to_owned(),
        }
    }
    /**CalculateLoyaltyPoints

Calculates the number of points a buyer can earn from a purchase. Applications might call this endpoint
to display the points to the buyer.

- If you are using the Orders API to manage orders, provide the `order_id` and (optional) `loyalty_account_id`.
Square reads the order to compute the points earned from the base loyalty program and an associated
[loyalty promotion](entity:LoyaltyPromotion).

- If you are not using the Orders API to manage orders, provide `transaction_amount_money` with the
purchase amount. Square uses this amount to calculate the points earned from the base loyalty program,
but not points earned from a loyalty promotion. For spend-based and visit-based programs, the `tax_mode`
setting of the accrual rule indicates how taxes should be treated for loyalty points accrual.
If the purchase qualifies for program points, call
[ListLoyaltyPromotions](api-endpoint:Loyalty-ListLoyaltyPromotions) and perform a client-side computation
to calculate whether the purchase also qualifies for promotion points. For more information, see
[Calculating promotion points](https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#calculate-promotion-points).

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-points#calculate-loyalty-points>.*/
    pub fn calculate_loyalty_points(
        &self,
        program_id: &str,
    ) -> request::CalculateLoyaltyPointsRequest {
        request::CalculateLoyaltyPointsRequest {
            http_client: &self,
            loyalty_account_id: None,
            order_id: None,
            program_id: program_id.to_owned(),
            transaction_amount_money: None,
        }
    }
    /**ListLoyaltyPromotions

Lists the loyalty promotions associated with a [loyalty program](entity:LoyaltyProgram).
Results are sorted by the `created_at` date in descending order (newest to oldest).

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#list-loyalty-promotions>.*/
    pub fn list_loyalty_promotions(
        &self,
        program_id: &str,
    ) -> request::ListLoyaltyPromotionsRequest {
        request::ListLoyaltyPromotionsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            program_id: program_id.to_owned(),
            status: None,
        }
    }
    /**CreateLoyaltyPromotion

Creates a loyalty promotion for a [loyalty program](entity:LoyaltyProgram). A loyalty promotion
enables buyers to earn points in addition to those earned from the base loyalty program.

This endpoint sets the loyalty promotion to the `ACTIVE` or `SCHEDULED` status, depending on the
`available_time` setting. A loyalty program can have a maximum of 10 loyalty promotions with an
`ACTIVE` or `SCHEDULED` status.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#create-loyalty-promotion>.*/
    pub fn create_loyalty_promotion(
        &self,
        idempotency_key: &str,
        loyalty_promotion: LoyaltyPromotion,
        program_id: &str,
    ) -> request::CreateLoyaltyPromotionRequest {
        request::CreateLoyaltyPromotionRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            loyalty_promotion,
            program_id: program_id.to_owned(),
        }
    }
    /**RetrieveLoyaltyPromotion

Retrieves a loyalty promotion.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#retrieve-loyalty-promotion>.*/
    pub fn retrieve_loyalty_promotion(
        &self,
        program_id: &str,
        promotion_id: &str,
    ) -> request::RetrieveLoyaltyPromotionRequest {
        request::RetrieveLoyaltyPromotionRequest {
            http_client: &self,
            program_id: program_id.to_owned(),
            promotion_id: promotion_id.to_owned(),
        }
    }
    /**CancelLoyaltyPromotion

Cancels a loyalty promotion. Use this endpoint to cancel an `ACTIVE` promotion earlier than the
end date, cancel an `ACTIVE` promotion when an end date is not specified, or cancel a `SCHEDULED` promotion.
Because updating a promotion is not supported, you can also use this endpoint to cancel a promotion before
you create a new one.

This endpoint sets the loyalty promotion to the `CANCELED` state

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-promotions#cancel-loyalty-promotion>.*/
    pub fn cancel_loyalty_promotion(
        &self,
        program_id: &str,
        promotion_id: &str,
    ) -> request::CancelLoyaltyPromotionRequest {
        request::CancelLoyaltyPromotionRequest {
            http_client: &self,
            program_id: program_id.to_owned(),
            promotion_id: promotion_id.to_owned(),
        }
    }
    /**CreateLoyaltyReward

Creates a loyalty reward. In the process, the endpoint does following:

- Uses the `reward_tier_id` in the request to determine the number of points
to lock for this reward.
- If the request includes `order_id`, it adds the reward and related discount to the order.

After a reward is created, the points are locked and
not available for the buyer to redeem another reward.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-rewards#create-loyalty-reward>.*/
    pub fn create_loyalty_reward(
        &self,
        idempotency_key: &str,
        reward: LoyaltyReward,
    ) -> request::CreateLoyaltyRewardRequest {
        request::CreateLoyaltyRewardRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            reward,
        }
    }
    /**SearchLoyaltyRewards

Searches for loyalty rewards. This endpoint accepts a request with no query filters and returns results for all loyalty accounts.
If you include a `query` object, `loyalty_account_id` is required and `status` is  optional.

If you know a reward ID, use the
[RetrieveLoyaltyReward](api-endpoint:Loyalty-RetrieveLoyaltyReward) endpoint.

Search results are sorted by `updated_at` in descending order.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-rewards#search-loyalty-rewards>.*/
    pub fn search_loyalty_rewards(&self) -> request::SearchLoyaltyRewardsRequest {
        request::SearchLoyaltyRewardsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**RetrieveLoyaltyReward

Retrieves a loyalty reward.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-rewards#retrieve-loyalty-reward>.*/
    pub fn retrieve_loyalty_reward(
        &self,
        reward_id: &str,
    ) -> request::RetrieveLoyaltyRewardRequest {
        request::RetrieveLoyaltyRewardRequest {
            http_client: &self,
            reward_id: reward_id.to_owned(),
        }
    }
    /**DeleteLoyaltyReward

Deletes a loyalty reward by doing the following:

- Returns the loyalty points back to the loyalty account.
- If an order ID was specified when the reward was created
(see [CreateLoyaltyReward](api-endpoint:Loyalty-CreateLoyaltyReward)),
it updates the order by removing the reward and related
discounts.

You cannot delete a reward that has reached the terminal state (REDEEMED).

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-rewards#delete-loyalty-reward>.*/
    pub fn delete_loyalty_reward(
        &self,
        reward_id: &str,
    ) -> request::DeleteLoyaltyRewardRequest {
        request::DeleteLoyaltyRewardRequest {
            http_client: &self,
            reward_id: reward_id.to_owned(),
        }
    }
    /**RedeemLoyaltyReward

Redeems a loyalty reward.

The endpoint sets the reward to the `REDEEMED` terminal state.

If you are using your own order processing system (not using the
Orders API), you call this endpoint after the buyer paid for the
purchase.

After the reward reaches the terminal state, it cannot be deleted.
In other words, points used for the reward cannot be returned
to the account.

See endpoint docs at <https://developer.squareup.com/docs/loyalty-api/loyalty-rewards#redeem-loyalty-reward>.*/
    pub fn redeem_loyalty_reward(
        &self,
        idempotency_key: &str,
        location_id: &str,
        reward_id: &str,
    ) -> request::RedeemLoyaltyRewardRequest {
        request::RedeemLoyaltyRewardRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            location_id: location_id.to_owned(),
            reward_id: reward_id.to_owned(),
        }
    }
    /**ListMerchants

Provides details about the merchant associated with a given access token.

The access token used to connect your application to a Square seller is associated
with a single merchant. That means that `ListMerchants` returns a list
with a single `Merchant` object. You can specify your personal access token
to get your own merchant information or specify an OAuth token to get the
information for the merchant that granted your application access.

If you know the merchant ID, you can also use the [RetrieveMerchant](api-endpoint:Merchants-RetrieveMerchant)
endpoint to retrieve the merchant information.

See endpoint docs at <https://developer.squareup.com/docs/merchants-api>.*/
    pub fn list_merchants(&self) -> request::ListMerchantsRequest {
        request::ListMerchantsRequest {
            http_client: &self,
            cursor: None,
        }
    }
    /**RetrieveMerchant

Retrieves the `Merchant` object for the given `merchant_id`.

See endpoint docs at <https://developer.squareup.com/docs/merchants-api>.*/
    pub fn retrieve_merchant(
        &self,
        merchant_id: &str,
    ) -> request::RetrieveMerchantRequest {
        request::RetrieveMerchantRequest {
            http_client: &self,
            merchant_id: merchant_id.to_owned(),
        }
    }
    /**ListPaymentLinks

Lists all payment links.

See endpoint docs at <https://developer.squareup.com/docs/checkout-api/manage-checkout>.*/
    pub fn list_payment_links(&self) -> request::ListPaymentLinksRequest {
        request::ListPaymentLinksRequest {
            http_client: &self,
            cursor: None,
            limit: None,
        }
    }
    /**CreatePaymentLink

Creates a Square-hosted checkout page. Applications can share the resulting payment link with their buyer to pay for goods and services.

See endpoint docs at <https://developer.squareup.com/docs/checkout-api>.*/
    pub fn create_payment_link(&self) -> request::CreatePaymentLinkRequest {
        request::CreatePaymentLinkRequest {
            http_client: &self,
            checkout_options: None,
            description: None,
            idempotency_key: None,
            order: None,
            payment_note: None,
            pre_populated_data: None,
            quick_pay: None,
        }
    }
    /**RetrievePaymentLink

Retrieves a payment link.

See endpoint docs at <https://developer.squareup.com/docs/checkout-api/manage-checkout>.*/
    pub fn retrieve_payment_link(
        &self,
        id: &str,
    ) -> request::RetrievePaymentLinkRequest {
        request::RetrievePaymentLinkRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**UpdatePaymentLink

Updates a payment link. You can update the `payment_link` fields such as
`description`, `checkout_options`, and  `pre_populated_data`.
You cannot update other fields such as the `order_id`, `version`, `URL`, or `timestamp` field.

See endpoint docs at <https://developer.squareup.com/docs/checkout-api/manage-checkout#update-a-payment-link>.*/
    pub fn update_payment_link(
        &self,
        id: &str,
        payment_link: PaymentLink,
    ) -> request::UpdatePaymentLinkRequest {
        request::UpdatePaymentLinkRequest {
            http_client: &self,
            id: id.to_owned(),
            payment_link,
        }
    }
    /**DeletePaymentLink

Deletes a payment link.

See endpoint docs at <https://developer.squareup.com/docs/checkout-api/manage-checkout>.*/
    pub fn delete_payment_link(&self, id: &str) -> request::DeletePaymentLinkRequest {
        request::DeletePaymentLinkRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**CreateOrder

Creates a new [order](entity:Order) that can include information about products for
purchase and settings to apply to the purchase.

To pay for a created order, see
[Pay for Orders](https://developer.squareup.com/docs/orders-api/pay-for-orders).

You can modify open orders using the [UpdateOrder](api-endpoint:Orders-UpdateOrder) endpoint.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/create-orders>.*/
    pub fn create_order(&self) -> request::CreateOrderRequest {
        request::CreateOrderRequest {
            http_client: &self,
            idempotency_key: None,
            order: None,
        }
    }
    /**BatchRetrieveOrders

Retrieves a set of [orders](entity:Order) by their IDs.

If a given order ID does not exist, the ID is ignored instead of generating an error.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/manage-orders#retrieve-orders>.*/
    pub fn batch_retrieve_orders(
        &self,
        order_ids: &[&str],
    ) -> request::BatchRetrieveOrdersRequest {
        request::BatchRetrieveOrdersRequest {
            http_client: &self,
            location_id: None,
            order_ids: order_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**CalculateOrder

Enables applications to preview order pricing without creating an order.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/create-orders#calculate-order>.*/
    pub fn calculate_order(&self, order: Order) -> request::CalculateOrderRequest {
        request::CalculateOrderRequest {
            http_client: &self,
            order,
            proposed_rewards: None,
        }
    }
    /**CloneOrder

Creates a new order, in the `DRAFT` state, by duplicating an existing order. The newly created order has
only the core fields (such as line items, taxes, and discounts) copied from the original order.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/create-orders#clone-an-order>.*/
    pub fn clone_order(&self, order_id: &str) -> request::CloneOrderRequest {
        request::CloneOrderRequest {
            http_client: &self,
            idempotency_key: None,
            order_id: order_id.to_owned(),
            version: None,
        }
    }
    /**SearchOrders

Search all orders for one or more locations. Orders include all sales,
returns, and exchanges regardless of how or when they entered the Square
ecosystem (such as Point of Sale, Invoices, and Connect APIs).

`SearchOrders` requests need to specify which locations to search and define a
[SearchOrdersQuery](entity:SearchOrdersQuery) object that controls
how to sort or filter the results. Your `SearchOrdersQuery` can:

  Set filter criteria.
  Set the sort order.
  Determine whether to return results as complete `Order` objects or as
[OrderEntry](entity:OrderEntry) objects.

Note that details for orders processed with Square Point of Sale while in
offline mode might not be transmitted to Square for up to 72 hours. Offline
orders have a `created_at` value that reflects the time the order was created,
not the time it was subsequently transmitted to Square.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/manage-orders#search-orders>.*/
    pub fn search_orders(&self) -> request::SearchOrdersRequest {
        request::SearchOrdersRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            location_ids: None,
            query: None,
            return_entries: None,
        }
    }
    /**RetrieveOrder

Retrieves an [Order](entity:Order) by ID.*/
    pub fn retrieve_order(&self, order_id: &str) -> request::RetrieveOrderRequest {
        request::RetrieveOrderRequest {
            http_client: &self,
            order_id: order_id.to_owned(),
        }
    }
    /**UpdateOrder

Updates an open [order](entity:Order) by adding, replacing, or deleting
fields. Orders with a `COMPLETED` or `CANCELED` state cannot be updated.

An `UpdateOrder` request requires the following:

- The `order_id` in the endpoint path, identifying the order to update.
- The latest `version` of the order to update.
- The [sparse order](https://developer.squareup.com/docs/orders-api/manage-orders/update-orders#sparse-order-objects)
containing only the fields to update and the version to which the update is
being applied.
- If deleting fields, the [dot notation paths](https://developer.squareup.com/docs/orders-api/manage-orders#on-dot-notation)
identifying the fields to clear.

To pay for an order, see
[Pay for Orders](https://developer.squareup.com/docs/orders-api/pay-for-orders).

See endpoint docs at <https://developer.squareup.com/docs/orders-api/manage-orders#update-orders>.*/
    pub fn update_order(&self, order_id: &str) -> request::UpdateOrderRequest {
        request::UpdateOrderRequest {
            http_client: &self,
            fields_to_clear: None,
            idempotency_key: None,
            order: None,
            order_id: order_id.to_owned(),
        }
    }
    /**PayOrder

Pay for an [order](entity:Order) using one or more approved [payments](entity:Payment)
or settle an order with a total of `0`.

The total of the `payment_ids` listed in the request must be equal to the order
total. Orders with a total amount of `0` can be marked as paid by specifying an empty
array of `payment_ids` in the request.

To be used with `PayOrder`, a payment must:

- Reference the order by specifying the `order_id` when [creating the payment](api-endpoint:Payments-CreatePayment).
Any approved payments that reference the same `order_id` not specified in the
`payment_ids` is canceled.
- Be approved with [delayed capture](https://developer.squareup.com/docs/payments-api/take-payments/card-payments/delayed-capture).
Using a delayed capture payment with `PayOrder` completes the approved payment.

See endpoint docs at <https://developer.squareup.com/docs/orders-api/pay-for-orders>.*/
    pub fn pay_order(
        &self,
        idempotency_key: &str,
        order_id: &str,
    ) -> request::PayOrderRequest {
        request::PayOrderRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            order_id: order_id.to_owned(),
            order_version: None,
            payment_ids: None,
        }
    }
    /**ListPayments

Retrieves a list of payments taken by the account making the request.

Results are eventually consistent, and new payments or changes to payments might take several
seconds to appear.

The maximum results per page is 100.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/retrieve-payments>.*/
    pub fn list_payments(&self) -> request::ListPaymentsRequest {
        request::ListPaymentsRequest {
            http_client: &self,
            begin_time: None,
            card_brand: None,
            cursor: None,
            end_time: None,
            last4: None,
            limit: None,
            location_id: None,
            sort_order: None,
            total: None,
        }
    }
    /**CreatePayment

Creates a payment using the provided source. You can use this endpoint
to charge a card (credit/debit card or
Square gift card) or record a payment that the seller received outside of Square
(cash payment from a buyer or a payment that an external entity
processed on behalf of the seller).

The endpoint creates a
`Payment` object and returns it in the response.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/take-payments>.*/
    pub fn create_payment(
        &self,
        amount_money: Money,
        idempotency_key: &str,
        source_id: &str,
    ) -> request::CreatePaymentRequest {
        request::CreatePaymentRequest {
            http_client: &self,
            accept_partial_authorization: None,
            amount_money,
            app_fee_money: None,
            autocomplete: None,
            billing_address: None,
            buyer_email_address: None,
            cash_details: None,
            customer_id: None,
            delay_action: None,
            delay_duration: None,
            external_details: None,
            idempotency_key: idempotency_key.to_owned(),
            location_id: None,
            note: None,
            order_id: None,
            reference_id: None,
            shipping_address: None,
            source_id: source_id.to_owned(),
            statement_description_identifier: None,
            team_member_id: None,
            tip_money: None,
            verification_token: None,
        }
    }
    /**CancelPaymentByIdempotencyKey

Cancels (voids) a payment identified by the idempotency key that is specified in the
request.

Use this method when the status of a `CreatePayment` request is unknown (for example, after you send a
`CreatePayment` request, a network error occurs and you do not get a response). In this case, you can
direct Square to cancel the payment using this endpoint. In the request, you provide the same
idempotency key that you provided in your `CreatePayment` request that you want to cancel. After
canceling the payment, you can submit your `CreatePayment` request again.

Note that if no payment with the specified idempotency key is found, no action is taken and the endpoint
returns successfully.*/
    pub fn cancel_payment_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> request::CancelPaymentByIdempotencyKeyRequest {
        request::CancelPaymentByIdempotencyKeyRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**GetPayment

Retrieves details for a specific payment.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/retrieve-payments>.*/
    pub fn get_payment(&self, payment_id: &str) -> request::GetPaymentRequest {
        request::GetPaymentRequest {
            http_client: &self,
            payment_id: payment_id.to_owned(),
        }
    }
    /**UpdatePayment

Updates a payment with the APPROVED status.
You can update the `amount_money` and `tip_money` using this endpoint.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/update-payments>.*/
    pub fn update_payment(
        &self,
        idempotency_key: &str,
        payment_id: &str,
    ) -> request::UpdatePaymentRequest {
        request::UpdatePaymentRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            payment: None,
            payment_id: payment_id.to_owned(),
        }
    }
    /**CancelPayment

Cancels (voids) a payment. You can use this endpoint to cancel a payment with
the APPROVED `status`.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/take-payments/card-payments#delayed-capture-of-a-card-payment>.*/
    pub fn cancel_payment(&self, payment_id: &str) -> request::CancelPaymentRequest {
        request::CancelPaymentRequest {
            http_client: &self,
            payment_id: payment_id.to_owned(),
        }
    }
    /**CompletePayment

Completes (captures) a payment.
By default, payments are set to complete immediately after they are created.

You can use this endpoint to complete a payment with the APPROVED `status`.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/take-payments/card-payments#delayed-capture-of-a-card-payment>.*/
    pub fn complete_payment(&self, payment_id: &str) -> request::CompletePaymentRequest {
        request::CompletePaymentRequest {
            http_client: &self,
            payment_id: payment_id.to_owned(),
            version_token: None,
        }
    }
    /**ListPayouts

Retrieves a list of all payouts for the default location.
You can filter payouts by location ID, status, time range, and order them in ascending or descending order.
To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

See endpoint docs at <https://developer.squareup.com/docs/payouts-api/overview#list-payouts>.*/
    pub fn list_payouts(&self) -> request::ListPayoutsRequest {
        request::ListPayoutsRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            end_time: None,
            limit: None,
            location_id: None,
            sort_order: None,
            status: None,
        }
    }
    /**GetPayout

Retrieves details of a specific payout identified by a payout ID.
To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

See endpoint docs at <https://developer.squareup.com/docs/payouts-api/overview#get-payout>.*/
    pub fn get_payout(&self, payout_id: &str) -> request::GetPayoutRequest {
        request::GetPayoutRequest {
            http_client: &self,
            payout_id: payout_id.to_owned(),
        }
    }
    /**ListPayoutEntries

Retrieves a list of all payout entries for a specific payout.
To call this endpoint, set `PAYOUTS_READ` for the OAuth scope.

See endpoint docs at <https://developer.squareup.com/docs/payouts-api/overview#list-payout-entries>.*/
    pub fn list_payout_entries(
        &self,
        payout_id: &str,
    ) -> request::ListPayoutEntriesRequest {
        request::ListPayoutEntriesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            payout_id: payout_id.to_owned(),
            sort_order: None,
        }
    }
    /**ListPaymentRefunds

Retrieves a list of refunds for the account making the request.

Results are eventually consistent, and new refunds or changes to refunds might take several
seconds to appear.

The maximum results per page is 100.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/retrieve-payments>.*/
    pub fn list_payment_refunds(&self) -> request::ListPaymentRefundsRequest {
        request::ListPaymentRefundsRequest {
            http_client: &self,
            begin_time: None,
            cursor: None,
            end_time: None,
            limit: None,
            location_id: None,
            sort_order: None,
            source_type: None,
            status: None,
        }
    }
    /**RefundPayment

Refunds a payment. You can refund the entire payment amount or a
portion of it. You can use this endpoint to refund a card payment or record a
refund of a cash or external payment. For more information, see
[Refund Payment](https://developer.squareup.com/docs/payments-api/refund-payments).

See endpoint docs at <https://developer.squareup.com/docs/payments-api/refund-payments#refund-a-payment>.*/
    pub fn refund_payment(
        &self,
        amount_money: Money,
        idempotency_key: &str,
    ) -> request::RefundPaymentRequest {
        request::RefundPaymentRequest {
            http_client: &self,
            amount_money,
            app_fee_money: None,
            idempotency_key: idempotency_key.to_owned(),
            payment_id: None,
            payment_version_token: None,
            reason: None,
            team_member_id: None,
        }
    }
    /**GetPaymentRefund

Retrieves a specific refund using the `refund_id`.

See endpoint docs at <https://developer.squareup.com/docs/payments-api/refund-payments#retrieve-refund-information>.*/
    pub fn get_payment_refund(
        &self,
        refund_id: &str,
    ) -> request::GetPaymentRefundRequest {
        request::GetPaymentRefundRequest {
            http_client: &self,
            refund_id: refund_id.to_owned(),
        }
    }
    /**ListSites

Lists the Square Online sites that belong to a seller. Sites are listed in descending order by the `created_at` date.


__Note:__ Square Online APIs are publicly available as part of an early access program. For more information, see [Early access program for Square Online APIs](https://developer.squareup.com/docs/online-api#early-access-program-for-square-online-apis).

See endpoint docs at <https://developer.squareup.com/docs/sites-api/use-the-api#list-sites>.*/
    pub fn list_sites(&self) -> request::ListSitesRequest {
        request::ListSitesRequest {
            http_client: &self,
        }
    }
    /**RetrieveSnippet

Retrieves your snippet from a Square Online site. A site can contain snippets from multiple snippet applications, but you can retrieve only the snippet that was added by your application.

You can call [ListSites](api-endpoint:Sites-ListSites) to get the IDs of the sites that belong to a seller.


__Note:__ Square Online APIs are publicly available as part of an early access program. For more information, see [Early access program for Square Online APIs](https://developer.squareup.com/docs/online-api#early-access-program-for-square-online-apis).

See endpoint docs at <https://developer.squareup.com/docs/snippets-api/use-the-api#retrieve-snippet>.*/
    pub fn retrieve_snippet(&self, site_id: &str) -> request::RetrieveSnippetRequest {
        request::RetrieveSnippetRequest {
            http_client: &self,
            site_id: site_id.to_owned(),
        }
    }
    /**UpsertSnippet

Adds a snippet to a Square Online site or updates the existing snippet on the site.
The snippet code is appended to the end of the `head` element on every page of the site, except checkout pages. A snippet application can add one snippet to a given site.

You can call [ListSites](api-endpoint:Sites-ListSites) to get the IDs of the sites that belong to a seller.


__Note:__ Square Online APIs are publicly available as part of an early access program. For more information, see [Early access program for Square Online APIs](https://developer.squareup.com/docs/online-api#early-access-program-for-square-online-apis).

See endpoint docs at <https://developer.squareup.com/docs/snippets-api/use-the-api#upsert-snippet>.*/
    pub fn upsert_snippet(
        &self,
        site_id: &str,
        snippet: Snippet,
    ) -> request::UpsertSnippetRequest {
        request::UpsertSnippetRequest {
            http_client: &self,
            site_id: site_id.to_owned(),
            snippet,
        }
    }
    /**DeleteSnippet

Removes your snippet from a Square Online site.

You can call [ListSites](api-endpoint:Sites-ListSites) to get the IDs of the sites that belong to a seller.


__Note:__ Square Online APIs are publicly available as part of an early access program. For more information, see [Early access program for Square Online APIs](https://developer.squareup.com/docs/online-api#early-access-program-for-square-online-apis).

See endpoint docs at <https://developer.squareup.com/docs/snippets-api/use-the-api#delete-snippet>.*/
    pub fn delete_snippet(&self, site_id: &str) -> request::DeleteSnippetRequest {
        request::DeleteSnippetRequest {
            http_client: &self,
            site_id: site_id.to_owned(),
        }
    }
    /**CreateSubscription

Creates a subscription to a subscription plan by a customer.

If you provide a card on file in the request, Square charges the card for
the subscription. Otherwise, Square bills an invoice to the customer's email
address. The subscription starts immediately, unless the request includes
the optional `start_date`. Each individual subscription is associated with a particular location.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#create-subscriptions>.*/
    pub fn create_subscription(
        &self,
        customer_id: &str,
        location_id: &str,
        plan_id: &str,
    ) -> request::CreateSubscriptionRequest {
        request::CreateSubscriptionRequest {
            http_client: &self,
            canceled_date: None,
            card_id: None,
            customer_id: customer_id.to_owned(),
            idempotency_key: None,
            location_id: location_id.to_owned(),
            plan_id: plan_id.to_owned(),
            price_override_money: None,
            source: None,
            start_date: None,
            tax_percentage: None,
            timezone: None,
        }
    }
    /**SearchSubscriptions

Searches for subscriptions.

Results are ordered chronologically by subscription creation date. If
the request specifies more than one location ID,
the endpoint orders the result
by location ID, and then by creation date within each location. If no locations are given
in the query, all locations are searched.

You can also optionally specify `customer_ids` to search by customer.
If left unset, all customers
associated with the specified locations are returned.
If the request specifies customer IDs, the endpoint orders results
first by location, within location by customer ID, and within
customer by subscription creation date.

For more information, see
[Retrieve subscriptions](https://developer.squareup.com/docs/subscriptions-api/overview#retrieve-subscriptions).

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#retrieve-subscriptions>.*/
    pub fn search_subscriptions(&self) -> request::SearchSubscriptionsRequest {
        request::SearchSubscriptionsRequest {
            http_client: &self,
            cursor: None,
            include: None,
            limit: None,
            query: None,
        }
    }
    /**RetrieveSubscription

Retrieves a subscription.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#retrieve-subscriptions>.*/
    pub fn retrieve_subscription(
        &self,
        subscription_id: &str,
    ) -> request::RetrieveSubscriptionRequest {
        request::RetrieveSubscriptionRequest {
            http_client: &self,
            include: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**UpdateSubscription

Updates a subscription. You can set, modify, and clear the
`subscription` field values.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#update-subscriptions>.*/
    pub fn update_subscription(
        &self,
        subscription_id: &str,
    ) -> request::UpdateSubscriptionRequest {
        request::UpdateSubscriptionRequest {
            http_client: &self,
            subscription: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**DeleteSubscriptionAction

Deletes a scheduled action for a subscription.*/
    pub fn delete_subscription_action(
        &self,
        action_id: &str,
        subscription_id: &str,
    ) -> request::DeleteSubscriptionActionRequest {
        request::DeleteSubscriptionActionRequest {
            http_client: &self,
            action_id: action_id.to_owned(),
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**CancelSubscription

Schedules a `CANCEL` action to cancel an active subscription
by setting the `canceled_date` field to the end of the active billing period
and changing the subscription status from ACTIVE to CANCELED after this date.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#cancel-subscriptions>.*/
    pub fn cancel_subscription(
        &self,
        subscription_id: &str,
    ) -> request::CancelSubscriptionRequest {
        request::CancelSubscriptionRequest {
            http_client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**ListSubscriptionEvents

Lists all events for a specific subscription.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#subscription-events>.*/
    pub fn list_subscription_events(
        &self,
        subscription_id: &str,
    ) -> request::ListSubscriptionEventsRequest {
        request::ListSubscriptionEventsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**PauseSubscription

Schedules a `PAUSE` action to pause an active subscription.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#pause-subscriptions>.*/
    pub fn pause_subscription(
        &self,
        subscription_id: &str,
    ) -> request::PauseSubscriptionRequest {
        request::PauseSubscriptionRequest {
            http_client: &self,
            pause_cycle_duration: None,
            pause_effective_date: None,
            pause_reason: None,
            resume_change_timing: None,
            resume_effective_date: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**ResumeSubscription

Schedules a `RESUME` action to resume a paused or a deactivated subscription.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview#deactivated-subscriptions>.*/
    pub fn resume_subscription(
        &self,
        subscription_id: &str,
    ) -> request::ResumeSubscriptionRequest {
        request::ResumeSubscriptionRequest {
            http_client: &self,
            resume_change_timing: None,
            resume_effective_date: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**SwapPlan

Schedules a `SWAP_PLAN` action to swap a subscription plan in an existing subscription.

See endpoint docs at <https://developer.squareup.com/docs/subscriptions-api/overview>.*/
    pub fn swap_plan(
        &self,
        new_plan_id: &str,
        subscription_id: &str,
    ) -> request::SwapPlanRequest {
        request::SwapPlanRequest {
            http_client: &self,
            new_plan_id: new_plan_id.to_owned(),
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**CreateTeamMember

Creates a single `TeamMember` object. The `TeamMember` object is returned on successful creates.
You must provide the following values in your request to this endpoint:
- `given_name`
- `family_name`

Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#set-up-a-new-team-member>.*/
    pub fn create_team_member(&self) -> request::CreateTeamMemberRequest {
        request::CreateTeamMemberRequest {
            http_client: &self,
            idempotency_key: None,
            team_member: None,
        }
    }
    /**BulkCreateTeamMembers

Creates multiple `TeamMember` objects. The created `TeamMember` objects are returned on successful creates.
This process is non-transactional and processes as much of the request as possible. If one of the creates in
the request cannot be successfully processed, the request is not marked as failed, but the body of the response
contains explicit error information for the failed create.

Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#bulk-create-team-members).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#bulk-create-team-members>.*/
    pub fn bulk_create_team_members(
        &self,
        team_members: serde_json::Value,
    ) -> request::BulkCreateTeamMembersRequest {
        request::BulkCreateTeamMembersRequest {
            http_client: &self,
            team_members,
        }
    }
    /**BulkUpdateTeamMembers

Updates multiple `TeamMember` objects. The updated `TeamMember` objects are returned on successful updates.
This process is non-transactional and processes as much of the request as possible. If one of the updates in
the request cannot be successfully processed, the request is not marked as failed, but the body of the response
contains explicit error information for the failed update.
Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#bulk-update-team-members).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#bulk-update-team-members>.*/
    pub fn bulk_update_team_members(
        &self,
        team_members: serde_json::Value,
    ) -> request::BulkUpdateTeamMembersRequest {
        request::BulkUpdateTeamMembersRequest {
            http_client: &self,
            team_members,
        }
    }
    /**SearchTeamMembers

Returns a paginated list of `TeamMember` objects for a business.
The list can be filtered by the following:
- location IDs
- `status`

See endpoint docs at <https://developer.squareup.com/docs/team/integration#search-for-team-members>.*/
    pub fn search_team_members(&self) -> request::SearchTeamMembersRequest {
        request::SearchTeamMembersRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**RetrieveTeamMember

Retrieves a `TeamMember` object for the given `TeamMember.id`.
Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#retrieve-a-team-member).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#retrieve-a-team-member>.*/
    pub fn retrieve_team_member(
        &self,
        team_member_id: &str,
    ) -> request::RetrieveTeamMemberRequest {
        request::RetrieveTeamMemberRequest {
            http_client: &self,
            team_member_id: team_member_id.to_owned(),
        }
    }
    /**UpdateTeamMember

Updates a single `TeamMember` object. The `TeamMember` object is returned on successful updates.
Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#update-a-team-member).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#update-a-team-member>.*/
    pub fn update_team_member(
        &self,
        team_member_id: &str,
    ) -> request::UpdateTeamMemberRequest {
        request::UpdateTeamMemberRequest {
            http_client: &self,
            team_member: None,
            team_member_id: team_member_id.to_owned(),
        }
    }
    /**RetrieveWageSetting

Retrieves a `WageSetting` object for a team member specified
by `TeamMember.id`.
Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#retrievewagesetting).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#retrieve-a-wage-setting-for-a-team-member>.*/
    pub fn retrieve_wage_setting(
        &self,
        team_member_id: &str,
    ) -> request::RetrieveWageSettingRequest {
        request::RetrieveWageSettingRequest {
            http_client: &self,
            team_member_id: team_member_id.to_owned(),
        }
    }
    /**UpdateWageSetting

Creates or updates a `WageSetting` object. The object is created if a
`WageSetting` with the specified `team_member_id` does not exist. Otherwise,
it fully replaces the `WageSetting` object for the team member.
The `WageSetting` is returned on a successful update.
Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#create-or-update-a-wage-setting).

See endpoint docs at <https://developer.squareup.com/docs/team/integration#step-2-assign-a-job-title-and-wage>.*/
    pub fn update_wage_setting(
        &self,
        team_member_id: &str,
        wage_setting: WageSetting,
    ) -> request::UpdateWageSettingRequest {
        request::UpdateWageSettingRequest {
            http_client: &self,
            team_member_id: team_member_id.to_owned(),
            wage_setting,
        }
    }
    /**CreateTerminalAction

Creates a Terminal action request and sends it to the specified device.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#request-a-square-terminal-action>.*/
    pub fn create_terminal_action(
        &self,
        action: TerminalAction,
        idempotency_key: &str,
    ) -> request::CreateTerminalActionRequest {
        request::CreateTerminalActionRequest {
            http_client: &self,
            action,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**SearchTerminalActions

Retrieves a filtered list of Terminal action requests created by the account making the request. Terminal action requests are available for 30 days.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#search-for-terminal-action-requests>.*/
    pub fn search_terminal_actions(&self) -> request::SearchTerminalActionsRequest {
        request::SearchTerminalActionsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**GetTerminalAction

Retrieves a Terminal action request by `action_id`. Terminal action requests are available for 30 days.*/
    pub fn get_terminal_action(
        &self,
        action_id: &str,
    ) -> request::GetTerminalActionRequest {
        request::GetTerminalActionRequest {
            http_client: &self,
            action_id: action_id.to_owned(),
        }
    }
    /**CancelTerminalAction

Cancels a Terminal action request if the status of the request permits it.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#cancel-a-terminal-action>.*/
    pub fn cancel_terminal_action(
        &self,
        action_id: &str,
    ) -> request::CancelTerminalActionRequest {
        request::CancelTerminalActionRequest {
            http_client: &self,
            action_id: action_id.to_owned(),
        }
    }
    /**CreateTerminalCheckout

Creates a Terminal checkout request and sends it to the specified device to take a payment
for the requested amount.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#request-a-square-terminal-checkout>.*/
    pub fn create_terminal_checkout(
        &self,
        checkout: TerminalCheckout,
        idempotency_key: &str,
    ) -> request::CreateTerminalCheckoutRequest {
        request::CreateTerminalCheckoutRequest {
            http_client: &self,
            checkout,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**SearchTerminalCheckouts

Returns a filtered list of Terminal checkout requests created by the application making the request. Only Terminal checkout requests created for the merchant scoped to the OAuth token are returned. Terminal checkout requests are available for 30 days.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#search-for-terminal-checkout-requests>.*/
    pub fn search_terminal_checkouts(&self) -> request::SearchTerminalCheckoutsRequest {
        request::SearchTerminalCheckoutsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**GetTerminalCheckout

Retrieves a Terminal checkout request by `checkout_id`. Terminal checkout requests are available for 30 days.*/
    pub fn get_terminal_checkout(
        &self,
        checkout_id: &str,
    ) -> request::GetTerminalCheckoutRequest {
        request::GetTerminalCheckoutRequest {
            http_client: &self,
            checkout_id: checkout_id.to_owned(),
        }
    }
    /**CancelTerminalCheckout

Cancels a Terminal checkout request if the status of the request permits it.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-payments#cancel-a-terminal-checkout>.*/
    pub fn cancel_terminal_checkout(
        &self,
        checkout_id: &str,
    ) -> request::CancelTerminalCheckoutRequest {
        request::CancelTerminalCheckoutRequest {
            http_client: &self,
            checkout_id: checkout_id.to_owned(),
        }
    }
    /**CreateTerminalRefund

Creates a request to refund an Interac payment completed on a Square Terminal. Refunds for Interac payments on a Square Terminal are supported only for Interac debit cards in Canada. Other refunds for Terminal payments should use the Refunds API. For more information, see [Refunds API](api:Refunds).

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-refunds#create-a-terminal-refund>.*/
    pub fn create_terminal_refund(
        &self,
        idempotency_key: &str,
    ) -> request::CreateTerminalRefundRequest {
        request::CreateTerminalRefundRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            refund: None,
        }
    }
    /**SearchTerminalRefunds

Retrieves a filtered list of Interac Terminal refund requests created by the seller making the request. Terminal refund requests are available for 30 days.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-refunds#search-for-terminal-refunds>.*/
    pub fn search_terminal_refunds(&self) -> request::SearchTerminalRefundsRequest {
        request::SearchTerminalRefundsRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            query: None,
        }
    }
    /**GetTerminalRefund

Retrieves an Interac Terminal refund object by ID. Terminal refund objects are available for 30 days.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-refunds#get-a-terminal-refund>.*/
    pub fn get_terminal_refund(
        &self,
        terminal_refund_id: &str,
    ) -> request::GetTerminalRefundRequest {
        request::GetTerminalRefundRequest {
            http_client: &self,
            terminal_refund_id: terminal_refund_id.to_owned(),
        }
    }
    /**CancelTerminalRefund

Cancels an Interac Terminal refund request by refund request ID if the status of the request permits it.

See endpoint docs at <https://developer.squareup.com/docs/terminal-api/square-terminal-refunds#cancel-a-terminal-refund>.*/
    pub fn cancel_terminal_refund(
        &self,
        terminal_refund_id: &str,
    ) -> request::CancelTerminalRefundRequest {
        request::CancelTerminalRefundRequest {
            http_client: &self,
            terminal_refund_id: terminal_refund_id.to_owned(),
        }
    }
    /**BulkCreateVendors

Creates one or more [Vendor](entity:Vendor) objects to represent suppliers to a seller.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/create-vendors>.*/
    pub fn bulk_create_vendors(
        &self,
        vendors: serde_json::Value,
    ) -> request::BulkCreateVendorsRequest {
        request::BulkCreateVendorsRequest {
            http_client: &self,
            vendors,
        }
    }
    /**BulkRetrieveVendors

Retrieves one or more vendors of specified [Vendor](entity:Vendor) IDs.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/retrieve-vendors>.*/
    pub fn bulk_retrieve_vendors(&self) -> request::BulkRetrieveVendorsRequest {
        request::BulkRetrieveVendorsRequest {
            http_client: &self,
            vendor_ids: None,
        }
    }
    /**BulkUpdateVendors

Updates one or more of existing [Vendor](entity:Vendor) objects as suppliers to a seller.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/update-vendors>.*/
    pub fn bulk_update_vendors(
        &self,
        vendors: serde_json::Value,
    ) -> request::BulkUpdateVendorsRequest {
        request::BulkUpdateVendorsRequest {
            http_client: &self,
            vendors,
        }
    }
    /**CreateVendor

Creates a single [Vendor](entity:Vendor) object to represent a supplier to a seller.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/create-vendors>.*/
    pub fn create_vendor(&self, idempotency_key: &str) -> request::CreateVendorRequest {
        request::CreateVendorRequest {
            http_client: &self,
            idempotency_key: idempotency_key.to_owned(),
            vendor: None,
        }
    }
    /**SearchVendors

Searches for vendors using a filter against supported [Vendor](entity:Vendor) properties and a supported sorter.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/search-for-vendors>.*/
    pub fn search_vendors(&self) -> request::SearchVendorsRequest {
        request::SearchVendorsRequest {
            http_client: &self,
            cursor: None,
            filter: None,
            sort: None,
        }
    }
    /**RetrieveVendor

Retrieves the vendor of a specified [Vendor](entity:Vendor) ID.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/retrieve-vendors>.*/
    pub fn retrieve_vendor(&self, vendor_id: &str) -> request::RetrieveVendorRequest {
        request::RetrieveVendorRequest {
            http_client: &self,
            vendor_id: vendor_id.to_owned(),
        }
    }
    /**UpdateVendor

Updates an existing [Vendor](entity:Vendor) object as a supplier to a seller.

See endpoint docs at <https://developer.squareup.com/docs/vendors-api/update-vendors>.*/
    pub fn update_vendor(&self, vendor: Vendor) -> request::UpdateVendorRequest {
        request::UpdateVendorRequest {
            http_client: &self,
            idempotency_key: None,
            vendor,
        }
    }
    /**ListWebhookEventTypes

Lists all webhook event types that can be subscribed to.*/
    pub fn list_webhook_event_types(&self) -> request::ListWebhookEventTypesRequest {
        request::ListWebhookEventTypesRequest {
            http_client: &self,
            api_version: None,
        }
    }
    /**ListWebhookSubscriptions

Lists all webhook subscriptions owned by your application.*/
    pub fn list_webhook_subscriptions(
        &self,
    ) -> request::ListWebhookSubscriptionsRequest {
        request::ListWebhookSubscriptionsRequest {
            http_client: &self,
            cursor: None,
            include_disabled: None,
            limit: None,
            sort_order: None,
        }
    }
    /**CreateWebhookSubscription

Creates a webhook subscription.*/
    pub fn create_webhook_subscription(
        &self,
        subscription: WebhookSubscription,
    ) -> request::CreateWebhookSubscriptionRequest {
        request::CreateWebhookSubscriptionRequest {
            http_client: &self,
            idempotency_key: None,
            subscription,
        }
    }
    /**RetrieveWebhookSubscription

Retrieves a webhook subscription identified by its ID.*/
    pub fn retrieve_webhook_subscription(
        &self,
        subscription_id: &str,
    ) -> request::RetrieveWebhookSubscriptionRequest {
        request::RetrieveWebhookSubscriptionRequest {
            http_client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**UpdateWebhookSubscription

Updates a webhook subscription.*/
    pub fn update_webhook_subscription(
        &self,
        subscription_id: &str,
    ) -> request::UpdateWebhookSubscriptionRequest {
        request::UpdateWebhookSubscriptionRequest {
            http_client: &self,
            subscription: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**DeleteWebhookSubscription

Deletes a webhook subscription.*/
    pub fn delete_webhook_subscription(
        &self,
        subscription_id: &str,
    ) -> request::DeleteWebhookSubscriptionRequest {
        request::DeleteWebhookSubscriptionRequest {
            http_client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**UpdateWebhookSubscriptionSignatureKey

Updates a webhook subscription by replacing the existing signature key with a new one.*/
    pub fn update_webhook_subscription_signature_key(
        &self,
        subscription_id: &str,
    ) -> request::UpdateWebhookSubscriptionSignatureKeyRequest {
        request::UpdateWebhookSubscriptionSignatureKeyRequest {
            http_client: &self,
            idempotency_key: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**TestWebhookSubscription

Tests a webhook subscription by sending a test event to the notification URL.*/
    pub fn test_webhook_subscription(
        &self,
        subscription_id: &str,
    ) -> request::TestWebhookSubscriptionRequest {
        request::TestWebhookSubscriptionRequest {
            http_client: &self,
            event_type: None,
            subscription_id: subscription_id.to_owned(),
        }
    }
}