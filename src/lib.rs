//! [`DwollaClient`](struct.DwollaClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub struct DwollaClient {
    pub client: httpclient::Client,
}
impl DwollaClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new()
                .base_url(
                    std::env::var("DWOLLA_ENV")
                        .expect("Missing environment variable DWOLLA_ENV")
                        .as_str(),
                ),
        }
    }
}
impl DwollaClient {
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
    /**root

Entry point to the Dwolla API.*/
    pub fn root(&self) -> request::RootRequest {
        request::RootRequest {
            http_client: &self,
        }
    }
    /**id

Get account info by id.*/
    pub fn get_account(&self, id: &str) -> request::GetAccountRequest {
        request::GetAccountRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getAccountFundingSources

Get an account's funding sources.*/
    pub fn get_account_funding_sources(
        &self,
        id: &str,
    ) -> request::GetAccountFundingSourcesRequest {
        request::GetAccountFundingSourcesRequest {
            http_client: &self,
            id: id.to_owned(),
            removed: None,
        }
    }
    /**createFundingSourcesToken

Create an token that is capable of adding a financial institution for the given Dwolla Account.*/
    pub fn create_funding_sources_token(
        &self,
        id: &str,
    ) -> request::CreateFundingSourcesTokenRequest {
        request::CreateFundingSourcesTokenRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getAccountIavToken

Get iav token for account.*/
    pub fn get_account_iav_token(
        &self,
        id: &str,
        token_type: &str,
    ) -> request::GetAccountIavTokenRequest {
        request::GetAccountIavTokenRequest {
            http_client: &self,
            id: id.to_owned(),
            token_type: token_type.to_owned(),
        }
    }
    /**getByAccount

List mass payments for an account.*/
    pub fn get_account_mass_payments(
        &self,
        id: &str,
    ) -> request::GetAccountMassPaymentsRequest {
        request::GetAccountMassPaymentsRequest {
            http_client: &self,
            correlation_id: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
        }
    }
    /**getAccountTransfers

List and search transfers for an account.*/
    pub fn get_account_transfers(
        &self,
        id: &str,
    ) -> request::GetAccountTransfersRequest {
        request::GetAccountTransfersRequest {
            http_client: &self,
            correlation_id: None,
            end_amount: None,
            end_date: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
            start_amount: None,
            start_date: None,
            status: None,
        }
    }
    /**getById

Get an beneficial owner by ID*/
    pub fn get_beneficial_owner(&self, id: &str) -> request::GetBeneficialOwnerRequest {
        request::GetBeneficialOwnerRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**update

Update a beneficial owner.*/
    pub fn update_beneficial_owner(
        &self,
        id: &str,
    ) -> request::UpdateBeneficialOwnerRequest {
        request::UpdateBeneficialOwnerRequest {
            http_client: &self,
            idempotency_key: None,
            address: None,
            date_of_birth: None,
            first_name: None,
            id: id.to_owned(),
            last_name: None,
            passport: None,
            ssn: None,
        }
    }
    /**getBeneficiaOwnerDocuments

Get documents uploaded for owner.*/
    pub fn get_beneficia_owner_documents(
        &self,
        id: &str,
    ) -> request::GetBeneficiaOwnerDocumentsRequest {
        request::GetBeneficiaOwnerDocumentsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**uploadBeneficialOwnerDocument

Upload a verification document for a beneficial owner.*/
    pub fn upload_beneficial_owner_document(
        &self,
        id: &str,
    ) -> request::UploadBeneficialOwnerDocumentRequest {
        request::UploadBeneficialOwnerDocumentRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
        }
    }
    /**list

Get a list business classifications.*/
    pub fn list_business_classifications(
        &self,
    ) -> request::ListBusinessClassificationsRequest {
        request::ListBusinessClassificationsRequest {
            http_client: &self,
        }
    }
    /**getBusinessClassification

Get a business classification with a list of industry classifications.*/
    pub fn get_business_classification(
        &self,
        id: &str,
    ) -> request::GetBusinessClassificationRequest {
        request::GetBusinessClassificationRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**list

Get a list of customers.*/
    pub fn list_customers(&self) -> request::ListCustomersRequest {
        request::ListCustomersRequest {
            http_client: &self,
            limit: None,
            offset: None,
            search: None,
            status: None,
        }
    }
    /**create

Create a new customer.*/
    pub fn create_customer(
        &self,
        first_name: &str,
        last_name: &str,
    ) -> request::CreateCustomerRequest {
        request::CreateCustomerRequest {
            http_client: &self,
            idempotency_key: None,
            address1: None,
            address2: None,
            business_classification: None,
            business_name: None,
            business_type: None,
            city: None,
            correlation_id: None,
            date_of_birth: None,
            doing_business_as: None,
            ein: None,
            email: None,
            first_name: first_name.to_owned(),
            ip_address: None,
            last_name: last_name.to_owned(),
            phone: None,
            postal_code: None,
            ssn: None,
            state: None,
            type_: None,
            website: None,
        }
    }
    /**getCustomer

Get a customer by id*/
    pub fn get_customer(&self, id: &str) -> request::GetCustomerRequest {
        request::GetCustomerRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**updateCustomer

Update customer record. Personal customer records are re-verified upon update.*/
    pub fn update_customer(&self, id: &str) -> request::UpdateCustomerRequest {
        request::UpdateCustomerRequest {
            http_client: &self,
            idempotency_key: None,
            address1: None,
            address2: None,
            business_classification: None,
            business_name: None,
            business_type: None,
            city: None,
            controller: None,
            correlation_id: None,
            date_of_birth: None,
            doing_business_as: None,
            ein: None,
            email: None,
            first_name: None,
            id: id.to_owned(),
            ip_address: None,
            last_name: None,
            phone: None,
            postal_code: None,
            ssn: None,
            state: None,
            status: None,
            type_: None,
            website: None,
        }
    }
    /**addBeneficialOwner

Add a beneficial owner*/
    pub fn add_beneficial_owner(
        &self,
        args: request::AddBeneficialOwnerRequired,
    ) -> request::AddBeneficialOwnerRequest {
        request::AddBeneficialOwnerRequest {
            http_client: &self,
            address: args.address,
            date_of_birth: args.date_of_birth.to_owned(),
            first_name: args.first_name.to_owned(),
            id: args.id.to_owned(),
            last_name: args.last_name.to_owned(),
            passport: None,
            ssn: None,
        }
    }
    /**getOwnershipStatus

Get a customer's ownership certification status.*/
    pub fn get_ownership_status(&self, id: &str) -> request::GetOwnershipStatusRequest {
        request::GetOwnershipStatusRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**changeOwnershipStatus

Change ownership certification status.*/
    pub fn change_ownership_status(
        &self,
        id: &str,
        status: &str,
    ) -> request::ChangeOwnershipStatusRequest {
        request::ChangeOwnershipStatusRequest {
            http_client: &self,
            id: id.to_owned(),
            status: status.to_owned(),
        }
    }
    /**getCustomerDocuments

Get documents uploaded for customer.*/
    pub fn get_customer_documents(
        &self,
        id: &str,
    ) -> request::GetCustomerDocumentsRequest {
        request::GetCustomerDocumentsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**uploadDocument

Upload a verification document.*/
    pub fn upload_customer_document(
        &self,
        id: &str,
    ) -> request::UploadCustomerDocumentRequest {
        request::UploadCustomerDocumentRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
        }
    }
    /**getCustomerFundingSources

Get a customer's funding sources.*/
    pub fn get_customer_funding_sources(
        &self,
        id: &str,
    ) -> request::GetCustomerFundingSourcesRequest {
        request::GetCustomerFundingSourcesRequest {
            http_client: &self,
            id: id.to_owned(),
            removed: None,
        }
    }
    /**createCustomerFundingSource

Create a new funding source.*/
    pub fn create_customer_funding_source(
        &self,
        account_number: &str,
        id: &str,
        routing_number: &str,
    ) -> request::CreateCustomerFundingSourceRequest {
        request::CreateCustomerFundingSourceRequest {
            http_client: &self,
            idempotency_key: None,
            links: None,
            account_number: account_number.to_owned(),
            bank_account_type: None,
            channels: None,
            id: id.to_owned(),
            name: None,
            routing_number: routing_number.to_owned(),
            type_: None,
            verified: None,
        }
    }
    /**createFundingSourcesTokenForCustomer

Create an token that is capable of adding a financial institution for the given customer.*/
    pub fn create_customer_funding_sources_token(
        &self,
        id: &str,
    ) -> request::CreateCustomerFundingSourcesTokenRequest {
        request::CreateCustomerFundingSourcesTokenRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getCustomerIavToken

Get iav token for customer.*/
    pub fn get_customer_iav_token(
        &self,
        id: &str,
    ) -> request::GetCustomerIavTokenRequest {
        request::GetCustomerIavTokenRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getLabelsForCustomer

Get labels for customer.*/
    pub fn get_labels_for_customer(
        &self,
        id: &str,
    ) -> request::GetLabelsForCustomerRequest {
        request::GetLabelsForCustomerRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**createLabel

Create a label.*/
    pub fn create_label(&self, amount: Amount, id: &str) -> request::CreateLabelRequest {
        request::CreateLabelRequest {
            http_client: &self,
            idempotency_key: None,
            amount,
            id: id.to_owned(),
        }
    }
    /**getByCustomer

Get a customer's mass payments.*/
    pub fn get_customer_mass_payments(
        &self,
        id: &str,
    ) -> request::GetCustomerMassPaymentsRequest {
        request::GetCustomerMassPaymentsRequest {
            http_client: &self,
            correlation_id: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
        }
    }
    /**getNotifications

Get a customer's notifications.*/
    pub fn get_customer_notifications(
        &self,
        id: &str,
    ) -> request::GetCustomerNotificationsRequest {
        request::GetCustomerNotificationsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getCustomerTransfers

Get a customer's transfers.*/
    pub fn get_customer_transfers(
        &self,
        id: &str,
    ) -> request::GetCustomerTransfersRequest {
        request::GetCustomerTransfersRequest {
            http_client: &self,
            correlation_id: None,
            end_amount: None,
            end_date: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
            start_amount: None,
            start_date: None,
            status: None,
        }
    }
    /**getDocument

Get a document by id*/
    pub fn get_document(&self, id: &str) -> request::GetDocumentRequest {
        request::GetDocumentRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**events

List events.*/
    pub fn list_events(&self) -> request::ListEventsRequest {
        request::ListEventsRequest {
            http_client: &self,
            limit: None,
            offset: None,
        }
    }
    /**id

Get an event by id.*/
    pub fn get_event(&self, id: &str) -> request::GetEventRequest {
        request::GetEventRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**createFundingSource

Create a new funding source.*/
    pub fn create_account_funding_source(
        &self,
        account_number: &str,
        routing_number: &str,
    ) -> request::CreateAccountFundingSourceRequest {
        request::CreateAccountFundingSourceRequest {
            http_client: &self,
            idempotency_key: None,
            links: None,
            account_number: account_number.to_owned(),
            bank_account_type: None,
            channels: None,
            name: None,
            routing_number: routing_number.to_owned(),
            type_: None,
            verified: None,
        }
    }
    /**id

Get a funding source by id.*/
    pub fn get_funding_source(&self, id: &str) -> request::GetFundingSourceRequest {
        request::GetFundingSourceRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**update

Update a funding source.*/
    pub fn update_funding_source(
        &self,
        id: &str,
        name: &str,
    ) -> request::UpdateFundingSourceRequest {
        request::UpdateFundingSourceRequest {
            http_client: &self,
            idempotency_key: None,
            links: None,
            account_number: None,
            bank_account_type: None,
            id: id.to_owned(),
            name: name.to_owned(),
            routing_number: None,
        }
    }
    /**getBalance

Get the balance of a funding source.*/
    pub fn get_balance(&self, id: &str) -> request::GetBalanceRequest {
        request::GetBalanceRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**verifyMicroDepositsExist

Verify pending verifications exist.*/
    pub fn get_micro_deposits(&self, id: &str) -> request::GetMicroDepositsRequest {
        request::GetMicroDepositsRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**microDeposits

Initiate or verify micro deposits for bank verification.*/
    pub fn initiate_micro_deposits(
        &self,
        amount1: Amount,
        amount2: Amount,
        id: &str,
    ) -> request::InitiateMicroDepositsRequest {
        request::InitiateMicroDepositsRequest {
            http_client: &self,
            idempotency_key: None,
            amount1,
            amount2,
            id: id.to_owned(),
        }
    }
    /**Get a KBA session by id

Retrieve KBA Questions*/
    pub fn get_kba_session(&self, id: &str) -> request::GetKbaSessionRequest {
        request::GetKbaSessionRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**Attempt to answer KBA questions

Answer KBA question set*/
    pub fn answer_kba_questions(
        &self,
        answers: Vec<AnsweredKbaQuestion>,
        id: &str,
    ) -> request::AnswerKbaQuestionsRequest {
        request::AnswerKbaQuestionsRequest {
            http_client: &self,
            idempotency_key: None,
            answers,
            id: id.to_owned(),
        }
    }
    /**byId

Get a label by id.*/
    pub fn get_label(&self, id: &str) -> request::GetLabelRequest {
        request::GetLabelRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**removeLabel

Remove a label.*/
    pub fn remove_label(&self, id: &str) -> request::RemoveLabelRequest {
        request::RemoveLabelRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getLedgerEntriesForLabel

Get ledger entries by label id.*/
    pub fn get_ledger_entries_for_label(
        &self,
        id: &str,
    ) -> request::GetLedgerEntriesForLabelRequest {
        request::GetLedgerEntriesForLabelRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**addLedgerEntry

Add ledger entry.*/
    pub fn add_ledger_entry(
        &self,
        amount: Amount,
        id: &str,
    ) -> request::AddLedgerEntryRequest {
        request::AddLedgerEntryRequest {
            http_client: &self,
            idempotency_key: None,
            amount,
            id: id.to_owned(),
        }
    }
    /**reallocateLabel

Reallocate two labels.*/
    pub fn reallocate_label(
        &self,
        args: request::ReallocateLabelRequired,
    ) -> request::ReallocateLabelRequest {
        request::ReallocateLabelRequest {
            http_client: &self,
            idempotency_key: None,
            amount: args.amount,
            from_label_id: args.from_label_id.to_owned(),
            partner_id: args.partner_id.to_owned(),
            to_label_id: args.to_label_id.to_owned(),
        }
    }
    /**getLabelReallocation

Get label reallocation.*/
    pub fn get_label_reallocation(
        &self,
        id: &str,
    ) -> request::GetLabelReallocationRequest {
        request::GetLabelReallocationRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getLabelLedgerEntry

Get a ledger entry by id.*/
    pub fn by_id3(&self, id: &str) -> request::ById3Request {
        request::ById3Request {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**byId

Get a mass payment item by id.*/
    pub fn get_mass_payment_item(&self, id: &str) -> request::GetMassPaymentItemRequest {
        request::GetMassPaymentItemRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**create

Create a new mass payment.*/
    pub fn create_mass_payment(
        &self,
        links: serde_json::Value,
        items: Vec<MassPaymentItemRequestBody>,
    ) -> request::CreateMassPaymentRequest {
        request::CreateMassPaymentRequest {
            http_client: &self,
            idempotency_key: None,
            links,
            ach_details: None,
            correlation_id: None,
            items,
            metadata: None,
            status: None,
        }
    }
    /**byId

Get a mass payment by id.*/
    pub fn get_mass_payment(&self, id: &str) -> request::GetMassPaymentRequest {
        request::GetMassPaymentRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**update

Update a mass-payment.*/
    pub fn update_mass_payment(
        &self,
        id: &str,
        status: &str,
    ) -> request::UpdateMassPaymentRequest {
        request::UpdateMassPaymentRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
            status: status.to_owned(),
        }
    }
    /**getMassPaymentItems

Get a mass payment's items.*/
    pub fn get_mass_payment_items(
        &self,
        id: &str,
    ) -> request::GetMassPaymentItemsRequest {
        request::GetMassPaymentItemsRequest {
            http_client: &self,
            correlation_id: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
            status: None,
        }
    }
    /**byId

Get a notification by id.*/
    pub fn get_notification(&self, id: &str) -> request::GetNotificationRequest {
        request::GetNotificationRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**createAuthorization

Create a new on-demand authorization.*/
    pub fn create_authorization(&self) -> request::CreateAuthorizationRequest {
        request::CreateAuthorizationRequest {
            http_client: &self,
        }
    }
    /**simulations

Simulate ach processing*/
    pub fn sandbox_simulations(&self) -> request::SandboxSimulationsRequest {
        request::SandboxSimulationsRequest {
            http_client: &self,
        }
    }
    /**oauth

OAuth get token response*/
    pub fn o_auth(&self) -> request::OAuthRequest {
        request::OAuthRequest {
            http_client: &self,
        }
    }
    /**create

Create a new transfer.*/
    pub fn create_transfer(
        &self,
        links: serde_json::Value,
        amount: serde_json::Value,
    ) -> request::CreateTransferRequest {
        request::CreateTransferRequest {
            http_client: &self,
            idempotency_key: None,
            links,
            ach_details: None,
            amount,
            clearing: None,
            correlation_id: None,
            fees: None,
            imad: None,
            metadata: None,
            wire_instructions: None,
        }
    }
    /**byId

Get a transfer by id.*/
    pub fn get_transfer(&self, id: &str) -> request::GetTransferRequest {
        request::GetTransferRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**update

Update a transfer.*/
    pub fn update_transfer(
        &self,
        id: &str,
        status: &str,
    ) -> request::UpdateTransferRequest {
        request::UpdateTransferRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
            status: status.to_owned(),
        }
    }
    /**failureById

Get a bank transfer failure by transfer id.*/
    pub fn get_transfer_failure(&self, id: &str) -> request::GetTransferFailureRequest {
        request::GetTransferFailureRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getFeesBySource

Get a transfer's fees.*/
    pub fn get_fees_by_source(&self, id: &str) -> request::GetFeesBySourceRequest {
        request::GetFeesBySourceRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**list

Get the list of webhooks.*/
    pub fn list_webhoook_subscriptions(
        &self,
    ) -> request::ListWebhoookSubscriptionsRequest {
        request::ListWebhoookSubscriptionsRequest {
            http_client: &self,
        }
    }
    /**create

Create a new webhook subscription.*/
    pub fn create_webhook_subscription(
        &self,
        secret: &str,
        url: &str,
    ) -> request::CreateWebhookSubscriptionRequest {
        request::CreateWebhookSubscriptionRequest {
            http_client: &self,
            idempotency_key: None,
            secret: secret.to_owned(),
            url: url.to_owned(),
        }
    }
    /**id

Get a webhook subscription by id.*/
    pub fn get_webhook_subscription(
        &self,
        id: &str,
    ) -> request::GetWebhookSubscriptionRequest {
        request::GetWebhookSubscriptionRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**updateSubscription

Update a subscription by id.*/
    pub fn update_webhook_subscription(
        &self,
        id: &str,
        paused: bool,
    ) -> request::UpdateWebhookSubscriptionRequest {
        request::UpdateWebhookSubscriptionRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
            paused,
        }
    }
    /**deleteById

Delete a webhook subscription by id.*/
    pub fn delete_by_id(&self, id: &str) -> request::DeleteByIdRequest {
        request::DeleteByIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**getWebhooksForWebhoookSubscription

Get webhooks by subscription id.*/
    pub fn hooks_by_id(&self, id: &str) -> request::HooksByIdRequest {
        request::HooksByIdRequest {
            http_client: &self,
            end_date: None,
            id: id.to_owned(),
            limit: None,
            offset: None,
            start_date: None,
        }
    }
    /**id

Get a webhook by id.*/
    pub fn get_webhook(&self, id: &str) -> request::GetWebhookRequest {
        request::GetWebhookRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**retriesById

Get retries requested by webhook id.*/
    pub fn retries_by_id(&self, id: &str) -> request::RetriesByIdRequest {
        request::RetriesByIdRequest {
            http_client: &self,
            id: id.to_owned(),
        }
    }
    /**retryWebhook

Retry a webhook by id.*/
    pub fn retry_webhook(&self, id: &str) -> request::RetryWebhookRequest {
        request::RetryWebhookRequest {
            http_client: &self,
            idempotency_key: None,
            id: id.to_owned(),
        }
    }
}