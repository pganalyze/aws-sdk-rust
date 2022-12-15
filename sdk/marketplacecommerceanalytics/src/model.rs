// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `SupportDataSetType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let supportdatasettype = unimplemented!();
/// match supportdatasettype {
///     SupportDataSetType::CustomerSupportContactsData => { /* ... */ },
///     SupportDataSetType::TestCustomerSupportContactsData => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `supportdatasettype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `SupportDataSetType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `SupportDataSetType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `SupportDataSetType::NewFeature` is defined.
/// Specifically, when `supportdatasettype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `SupportDataSetType::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SupportDataSetType {
    #[allow(missing_docs)] // documentation missing in model
    CustomerSupportContactsData,
    #[allow(missing_docs)] // documentation missing in model
    TestCustomerSupportContactsData,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for SupportDataSetType {
    fn from(s: &str) -> Self {
        match s {
            "customer_support_contacts_data" => SupportDataSetType::CustomerSupportContactsData,
            "test_customer_support_contacts_data" => {
                SupportDataSetType::TestCustomerSupportContactsData
            }
            other => {
                SupportDataSetType::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for SupportDataSetType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SupportDataSetType::from(s))
    }
}
impl SupportDataSetType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SupportDataSetType::CustomerSupportContactsData => "customer_support_contacts_data",
            SupportDataSetType::TestCustomerSupportContactsData => {
                "test_customer_support_contacts_data"
            }
            SupportDataSetType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "customer_support_contacts_data",
            "test_customer_support_contacts_data",
        ]
    }
}
impl AsRef<str> for SupportDataSetType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `DataSetType`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let datasettype = unimplemented!();
/// match datasettype {
///     DataSetType::CustomerProfileByGeography => { /* ... */ },
///     DataSetType::CustomerProfileByIndustry => { /* ... */ },
///     DataSetType::CustomerProfileByRevenue => { /* ... */ },
///     DataSetType::CustomerSubscriberAnnualSubscriptions => { /* ... */ },
///     DataSetType::CustomerSubscriberHourlyMonthlySubscriptions => { /* ... */ },
///     DataSetType::DailyBusinessCanceledProductSubscribers => { /* ... */ },
///     DataSetType::DailyBusinessFees => { /* ... */ },
///     DataSetType::DailyBusinessFreeTrialConversions => { /* ... */ },
///     DataSetType::DailyBusinessNewInstances => { /* ... */ },
///     DataSetType::DailyBusinessNewProductSubscribers => { /* ... */ },
///     DataSetType::DailyBusinessUsageByInstanceType => { /* ... */ },
///     DataSetType::DisbursedAmountByAgeOfDisbursedFunds => { /* ... */ },
///     DataSetType::DisbursedAmountByAgeOfPastDueFunds => { /* ... */ },
///     DataSetType::DisbursedAmountByAgeOfUncollectedFunds => { /* ... */ },
///     DataSetType::DisbursedAmountByCustomerGeo => { /* ... */ },
///     DataSetType::DisbursedAmountByInstanceHours => { /* ... */ },
///     DataSetType::DisbursedAmountByProduct => { /* ... */ },
///     DataSetType::DisbursedAmountByProductWithUncollectedFunds => { /* ... */ },
///     DataSetType::DisbursedAmountByUncollectedFundsBreakdown => { /* ... */ },
///     DataSetType::MonthlyRevenueAnnualSubscriptions => { /* ... */ },
///     DataSetType::MonthlyRevenueBillingAndRevenueData => { /* ... */ },
///     DataSetType::MonthlyRevenueFieldDemonstrationUsage => { /* ... */ },
///     DataSetType::MonthlyRevenueFlexiblePaymentSchedule => { /* ... */ },
///     DataSetType::SalesCompensationBilledRevenue => { /* ... */ },
///     DataSetType::UsSalesAndUseTaxRecords => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `datasettype` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `DataSetType::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `DataSetType::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `DataSetType::NewFeature` is defined.
/// Specifically, when `datasettype` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `DataSetType::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DataSetType {
    #[allow(missing_docs)] // documentation missing in model
    CustomerProfileByGeography,
    #[allow(missing_docs)] // documentation missing in model
    CustomerProfileByIndustry,
    #[allow(missing_docs)] // documentation missing in model
    CustomerProfileByRevenue,
    #[allow(missing_docs)] // documentation missing in model
    CustomerSubscriberAnnualSubscriptions,
    #[allow(missing_docs)] // documentation missing in model
    CustomerSubscriberHourlyMonthlySubscriptions,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessCanceledProductSubscribers,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessFees,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessFreeTrialConversions,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessNewInstances,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessNewProductSubscribers,
    #[allow(missing_docs)] // documentation missing in model
    DailyBusinessUsageByInstanceType,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByAgeOfDisbursedFunds,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByAgeOfPastDueFunds,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByAgeOfUncollectedFunds,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByCustomerGeo,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByInstanceHours,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByProduct,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByProductWithUncollectedFunds,
    #[allow(missing_docs)] // documentation missing in model
    DisbursedAmountByUncollectedFundsBreakdown,
    #[allow(missing_docs)] // documentation missing in model
    MonthlyRevenueAnnualSubscriptions,
    #[allow(missing_docs)] // documentation missing in model
    MonthlyRevenueBillingAndRevenueData,
    #[allow(missing_docs)] // documentation missing in model
    MonthlyRevenueFieldDemonstrationUsage,
    #[allow(missing_docs)] // documentation missing in model
    MonthlyRevenueFlexiblePaymentSchedule,
    #[allow(missing_docs)] // documentation missing in model
    SalesCompensationBilledRevenue,
    #[allow(missing_docs)] // documentation missing in model
    UsSalesAndUseTaxRecords,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for DataSetType {
    fn from(s: &str) -> Self {
        match s {
            "customer_profile_by_geography" => DataSetType::CustomerProfileByGeography,
            "customer_profile_by_industry" => DataSetType::CustomerProfileByIndustry,
            "customer_profile_by_revenue" => DataSetType::CustomerProfileByRevenue,
            "customer_subscriber_annual_subscriptions" => {
                DataSetType::CustomerSubscriberAnnualSubscriptions
            }
            "customer_subscriber_hourly_monthly_subscriptions" => {
                DataSetType::CustomerSubscriberHourlyMonthlySubscriptions
            }
            "daily_business_canceled_product_subscribers" => {
                DataSetType::DailyBusinessCanceledProductSubscribers
            }
            "daily_business_fees" => DataSetType::DailyBusinessFees,
            "daily_business_free_trial_conversions" => {
                DataSetType::DailyBusinessFreeTrialConversions
            }
            "daily_business_new_instances" => DataSetType::DailyBusinessNewInstances,
            "daily_business_new_product_subscribers" => {
                DataSetType::DailyBusinessNewProductSubscribers
            }
            "daily_business_usage_by_instance_type" => {
                DataSetType::DailyBusinessUsageByInstanceType
            }
            "disbursed_amount_by_age_of_disbursed_funds" => {
                DataSetType::DisbursedAmountByAgeOfDisbursedFunds
            }
            "disbursed_amount_by_age_of_past_due_funds" => {
                DataSetType::DisbursedAmountByAgeOfPastDueFunds
            }
            "disbursed_amount_by_age_of_uncollected_funds" => {
                DataSetType::DisbursedAmountByAgeOfUncollectedFunds
            }
            "disbursed_amount_by_customer_geo" => DataSetType::DisbursedAmountByCustomerGeo,
            "disbursed_amount_by_instance_hours" => DataSetType::DisbursedAmountByInstanceHours,
            "disbursed_amount_by_product" => DataSetType::DisbursedAmountByProduct,
            "disbursed_amount_by_product_with_uncollected_funds" => {
                DataSetType::DisbursedAmountByProductWithUncollectedFunds
            }
            "disbursed_amount_by_uncollected_funds_breakdown" => {
                DataSetType::DisbursedAmountByUncollectedFundsBreakdown
            }
            "monthly_revenue_annual_subscriptions" => {
                DataSetType::MonthlyRevenueAnnualSubscriptions
            }
            "monthly_revenue_billing_and_revenue_data" => {
                DataSetType::MonthlyRevenueBillingAndRevenueData
            }
            "monthly_revenue_field_demonstration_usage" => {
                DataSetType::MonthlyRevenueFieldDemonstrationUsage
            }
            "monthly_revenue_flexible_payment_schedule" => {
                DataSetType::MonthlyRevenueFlexiblePaymentSchedule
            }
            "sales_compensation_billed_revenue" => DataSetType::SalesCompensationBilledRevenue,
            "us_sales_and_use_tax_records" => DataSetType::UsSalesAndUseTaxRecords,
            other => DataSetType::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for DataSetType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DataSetType::from(s))
    }
}
impl DataSetType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DataSetType::CustomerProfileByGeography => "customer_profile_by_geography",
            DataSetType::CustomerProfileByIndustry => "customer_profile_by_industry",
            DataSetType::CustomerProfileByRevenue => "customer_profile_by_revenue",
            DataSetType::CustomerSubscriberAnnualSubscriptions => {
                "customer_subscriber_annual_subscriptions"
            }
            DataSetType::CustomerSubscriberHourlyMonthlySubscriptions => {
                "customer_subscriber_hourly_monthly_subscriptions"
            }
            DataSetType::DailyBusinessCanceledProductSubscribers => {
                "daily_business_canceled_product_subscribers"
            }
            DataSetType::DailyBusinessFees => "daily_business_fees",
            DataSetType::DailyBusinessFreeTrialConversions => {
                "daily_business_free_trial_conversions"
            }
            DataSetType::DailyBusinessNewInstances => "daily_business_new_instances",
            DataSetType::DailyBusinessNewProductSubscribers => {
                "daily_business_new_product_subscribers"
            }
            DataSetType::DailyBusinessUsageByInstanceType => {
                "daily_business_usage_by_instance_type"
            }
            DataSetType::DisbursedAmountByAgeOfDisbursedFunds => {
                "disbursed_amount_by_age_of_disbursed_funds"
            }
            DataSetType::DisbursedAmountByAgeOfPastDueFunds => {
                "disbursed_amount_by_age_of_past_due_funds"
            }
            DataSetType::DisbursedAmountByAgeOfUncollectedFunds => {
                "disbursed_amount_by_age_of_uncollected_funds"
            }
            DataSetType::DisbursedAmountByCustomerGeo => "disbursed_amount_by_customer_geo",
            DataSetType::DisbursedAmountByInstanceHours => "disbursed_amount_by_instance_hours",
            DataSetType::DisbursedAmountByProduct => "disbursed_amount_by_product",
            DataSetType::DisbursedAmountByProductWithUncollectedFunds => {
                "disbursed_amount_by_product_with_uncollected_funds"
            }
            DataSetType::DisbursedAmountByUncollectedFundsBreakdown => {
                "disbursed_amount_by_uncollected_funds_breakdown"
            }
            DataSetType::MonthlyRevenueAnnualSubscriptions => {
                "monthly_revenue_annual_subscriptions"
            }
            DataSetType::MonthlyRevenueBillingAndRevenueData => {
                "monthly_revenue_billing_and_revenue_data"
            }
            DataSetType::MonthlyRevenueFieldDemonstrationUsage => {
                "monthly_revenue_field_demonstration_usage"
            }
            DataSetType::MonthlyRevenueFlexiblePaymentSchedule => {
                "monthly_revenue_flexible_payment_schedule"
            }
            DataSetType::SalesCompensationBilledRevenue => "sales_compensation_billed_revenue",
            DataSetType::UsSalesAndUseTaxRecords => "us_sales_and_use_tax_records",
            DataSetType::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "customer_profile_by_geography",
            "customer_profile_by_industry",
            "customer_profile_by_revenue",
            "customer_subscriber_annual_subscriptions",
            "customer_subscriber_hourly_monthly_subscriptions",
            "daily_business_canceled_product_subscribers",
            "daily_business_fees",
            "daily_business_free_trial_conversions",
            "daily_business_new_instances",
            "daily_business_new_product_subscribers",
            "daily_business_usage_by_instance_type",
            "disbursed_amount_by_age_of_disbursed_funds",
            "disbursed_amount_by_age_of_past_due_funds",
            "disbursed_amount_by_age_of_uncollected_funds",
            "disbursed_amount_by_customer_geo",
            "disbursed_amount_by_instance_hours",
            "disbursed_amount_by_product",
            "disbursed_amount_by_product_with_uncollected_funds",
            "disbursed_amount_by_uncollected_funds_breakdown",
            "monthly_revenue_annual_subscriptions",
            "monthly_revenue_billing_and_revenue_data",
            "monthly_revenue_field_demonstration_usage",
            "monthly_revenue_flexible_payment_schedule",
            "sales_compensation_billed_revenue",
            "us_sales_and_use_tax_records",
        ]
    }
}
impl AsRef<str> for DataSetType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
