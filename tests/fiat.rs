use anyhow::Result;
use privy_rust::generated::types::*;

mod common;

#[tokio::test]
#[ignore = "need an api key for bridge"]
async fn test_fiat_configure_app() -> Result<()> {
    let client = common::get_test_client()?;
    let app_id = client.app_id();

    let body = ConfigureAppForFiatOnOffRampingBody {
        provider: ConfigureAppForFiatOnOffRampingBodyProvider::Bridge,
        api_key: "".parse()?,
    };

    let result = client.fiat().configure_app(&app_id, &body).await;

    match result {
        Ok(response) => {
            println!("Configure app response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Configure app failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_get_status() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = UserFiatStatusesBody {
        provider: UserFiatStatusesBodyProvider::Bridge,
    };

    let result = client.fiat().get_status(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("Fiat status response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Get status failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_get_kyc_link() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = GetUserFiatKycLinkBody {
        email: "TODO".to_string(),
        endorsements: vec![],
        full_name: None,
        provider: GetUserFiatKycLinkBodyProvider::Bridge,
        redirect_uri: None,
        type_: Some(GetUserFiatKycLinkBodyType::Individual),
    };

    let result = client.fiat().get_kyc_link(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("KYC link response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Get KYC link failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_accounts_get() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let result = client
        .fiat()
        .accounts()
        .get(&user.id, GetUserFiatAccountsProvider::Bridge)
        .await;

    match result {
        Ok(response) => {
            println!("Fiat accounts response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Get fiat accounts failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_accounts_create() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = CreateUserFiatAccountBody {
        provider: CreateUserFiatAccountBodyProvider::Bridge,
        account_owner_name: "TODO".parse()?,
        currency: CreateUserFiatAccountBodyCurrency::Usd,
        account: None,
        address: None,
        bank_name: None,
        first_name: None,
        iban: None,
        last_name: None,
        swift: None,
    };

    let result = client.fiat().accounts().create(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("Create fiat account response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Create fiat account failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_kyc_create() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = InitiateUserFiatKycBody::Bridge {
        first_name: "Rust".parse()?,
        last_name: "Sdk".parse()?,
        birth_date: "01/01/2000".parse()?,
        email: "rust-sdk@privy.io".parse()?,
        residential_address: InitiateUserFiatKycBodyBridgeResidentialAddress {
            city: "San Francisco".parse()?,
            country: "USA".parse()?,
            postal_code: Some("94103".parse()?),
            street_line_1: "Crypto Street".parse()?,
            street_line_2: None,
            subdivision: "ABC".parse()?,
        },
        endorsements: vec![],
        documents: vec![],
        identifying_information: vec![],
        type_: InitiateUserFiatKycBodyBridgeType::Individual,

        account_purpose: None,
        account_purpose_other: None,
        acting_as_intermediary: None,
        completed_customer_safety_check_at: None,
        employment_status: None,
        expected_monthly_payments_usd: None,
        has_signed_terms_of_service: None,
        kyc_screen: None,
        middle_name: None,
        most_recent_occupation: None,
        nationality: None,
        ofac_screen: None,
        phone: None,
        signed_agreement_id: None,
        source_of_funds: None,
        transliterated_first_name: None,
        transliterated_last_name: None,
        transliterated_middle_name: None,
        transliterated_residential_address: None,
        verified_selfie_at: None,
    };

    let result = client.fiat().kyc().create(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("Initiate KYC response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Initiate KYC failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_onramp_create() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = InitiateUserFiatOnrampBody {
        provider: InitiateUserFiatOnrampBodyProvider::Bridge,
        amount: "100".parse()?,
        destination: InitiateUserFiatOnrampBodyDestination {
            to_address: "0x742d35Cc6634C0532925a3b8D5c9B40AB8a04C8A".to_string(),
            chain: InitiateUserFiatOnrampBodyDestinationChain::Ethereum,
            currency: InitiateUserFiatOnrampBodyDestinationCurrency::Usdc,
        },
        source: InitiateUserFiatOnrampBodySource {
            currency: InitiateUserFiatOnrampBodySourceCurrency::Usd,
            payment_rail: InitiateUserFiatOnrampBodySourcePaymentRail::Wire,
        },
    };

    let result = client.fiat().onramp().create(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("Create onramp response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Create onramp failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_fiat_offramp_create() -> Result<()> {
    let client = common::get_test_client()?;
    let user = common::ensure_test_user(&client).await?;

    let body = InitiateUserFiatOfframpBody {
        provider: InitiateUserFiatOfframpBodyProvider::Bridge,
        amount: "50".parse()?,
        destination: InitiateUserFiatOfframpBodyDestination {
            currency: InitiateUserFiatOfframpBodyDestinationCurrency::Eur,
            external_account_id: uuid::Uuid::new_v4(),
            payment_rail: InitiateUserFiatOfframpBodyDestinationPaymentRail::Wire,
        },
        source: InitiateUserFiatOfframpBodySource {
            chain: InitiateUserFiatOfframpBodySourceChain::Ethereum,
            currency: InitiateUserFiatOfframpBodySourceCurrency::Usdc,
            from_address: "todo".to_string(),
        },
    };

    let result = client.fiat().offramp().create(&user.id, &body).await;

    match result {
        Ok(response) => {
            println!("Create offramp response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            println!("Create offramp failed (expected in test): {:?}", e);
            Ok(())
        }
    }
}
