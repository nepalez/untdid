# untdid

The code lists of the UN/EDIFACT data elements from the United Nations Trade Data Interchange Directory (UNTDID), release D.24A.

<img src="https://cdn.evilmartians.com/badges/logo-no-label.svg" alt="Evil Martians logo" width="22" height="16" /> <b>untdid</b> is built by <b><a href="https://evilmartians.com/">Evil Martians</a></b>, an American design and engineering consultancy for <b>developer tools, AI, and cybersecurity startups</b>.

The code list of every data element becomes a Rust `enum`. The enums are generated at compile time from the official code list file `UNCL.24A` and embedded as `static` data. The crate makes no runtime allocations and performs no I/O.

The codes the directory marks for deletion are skipped.

## Installation

Every code list stays behind a feature of its own, so a build contains only the lists it enables.

```toml
[dependencies]
untdid = { version = "0.1", features = ["document-name", "payment-means"] }
```

## Code lists

| Type | Feature | Element | Name |
|---|---|---|---|
| `DocumentName` | `document-name` | 1001 | Document name code |
| `MessageSection` | `message-section` | 1049 | Message section code |
| `DocumentLineAction` | `document-line-action` | 1073 | Document line action code |
| `ReferenceQualifier` | `reference-qualifier` | 1153 | Reference code qualifier |
| `SequenceIdentifierSource` | `sequence-identifier-source` | 1159 | Sequence identifier source code |
| `MessageFunction` | `message-function` | 1225 | Message function code |
| `CalculationSequence` | `calculation-sequence` | 1227 | Calculation sequence code |
| `Action` | `action` | 1229 | Action code |
| `DocumentStatus` | `document-status` | 1373 | Document status code |
| `ComputerEnvironmentDetailsQualifier` | `computer-environment-details-qualifier` | 1501 | Computer environment details code qualifier |
| `DataFormatDescription` | `data-format-description` | 1503 | Data format description code |
| `ValueListType` | `value-list-type` | 1505 | Value list type code |
| `DesignatedClass` | `designated-class` | 1507 | Designated class code |
| `DateOrTimeOrPeriodFunctionQualifier` | `date-or-time-or-period-function-qualifier` | 2005 | Date or time or period function code qualifier |
| `TermsTimeRelation` | `terms-time-relation` | 2009 | Terms time relation code |
| `Frequency` | `frequency` | 2013 | Frequency code |
| `DespatchPattern` | `despatch-pattern` | 2015 | Despatch pattern code |
| `DespatchPatternTiming` | `despatch-pattern-timing` | 2017 | Despatch pattern timing code |
| `PeriodTypeQualifier` | `period-type-qualifier` | 2023 | Period type code qualifier |
| `PeriodType` | `period-type` | 2151 | Period type code |
| `ChargePeriodType` | `charge-period-type` | 2155 | Charge period type code |
| `DateOrTimeOrPeriodFormat` | `date-or-time-or-period-format` | 2379 | Date or time or period format code |
| `EventTimeReference` | `event-time-reference` | 2475 | Event time reference code |
| `PartyFunctionQualifier` | `party-function-qualifier` | 3035 | Party function code qualifier |
| `PartyNameFormat` | `party-name-format` | 3045 | Party name format code |
| `CodeListResponsibleAgency` | `code-list-responsible-agency` | 3055 | Code list responsible agency code |
| `TestMedium` | `test-medium` | 3077 | Test medium code |
| `OrganisationClassification` | `organisation-classification` | 3079 | Organisation classification code |
| `OrganisationalClassName` | `organisational-class-name` | 3083 | Organisational class name code |
| `AddressType` | `address-type` | 3131 | Address type code |
| `ContactFunction` | `contact-function` | 3139 | Contact function code |
| `CommunicationMediumType` | `communication-medium-type` | 3153 | Communication medium type code |
| `CommunicationMeansType` | `communication-means-type` | 3155 | Communication means type code |
| `LocationFunctionQualifier` | `location-function-qualifier` | 3227 | Location function code qualifier |
| `SampleLocationDescription` | `sample-location-description` | 3237 | Sample location description code |
| `GeographicArea` | `geographic-area` | 3279 | Geographic area code |
| `InstructionReceivingPartyIdentifier` | `instruction-receiving-party-identifier` | 3285 | Instruction receiving party identifier |
| `PersonCharacteristicQualifier` | `person-characteristic-qualifier` | 3289 | Person characteristic code qualifier |
| `NameOriginalAlphabet` | `name-original-alphabet` | 3295 | Name original alphabet code |
| `AddressPurpose` | `address-purpose` | 3299 | Address purpose code |
| `EnactingPartyIdentifier` | `enacting-party-identifier` | 3301 | Enacting party identifier |
| `NameStatus` | `name-status` | 3397 | Name status code |
| `NameComponentUsage` | `name-component-usage` | 3401 | Name component usage code |
| `NameType` | `name-type` | 3403 | Name type code |
| `NameComponentTypeQualifier` | `name-component-type-qualifier` | 3405 | Name component type code qualifier |
| `LanguageQualifier` | `language-qualifier` | 3455 | Language code qualifier |
| `OriginatorType` | `originator-type` | 3457 | Originator type code |
| `AddressStatus` | `address-status` | 3475 | Address status code |
| `AddressFormat` | `address-format` | 3477 | Address format code |
| `MaritalStatusDescription` | `marital-status-description` | 3479 | Marital status description code |
| `NationalityQualifier` | `nationality-qualifier` | 3493 | Nationality code qualifier |
| `DeliveryPlanCommitmentLevel` | `delivery-plan-commitment-level` | 4017 | Delivery plan commitment level code |
| `BusinessFunction` | `business-function` | 4025 | Business function code |
| `BusinessFunctionTypeQualifier` | `business-function-type-qualifier` | 4027 | Business function type code qualifier |
| `PriorityTypeQualifier` | `priority-type-qualifier` | 4035 | Priority type code qualifier |
| `PriorityDescription` | `priority-description` | 4037 | Priority description code |
| `TradeClass` | `trade-class` | 4043 | Trade class code |
| `CertaintyDescription` | `certainty-description` | 4049 | Certainty description code |
| `CharacteristicRelevance` | `characteristic-relevance` | 4051 | Characteristic relevance code |
| `DeliveryOrTransportTermsDescription` | `delivery-or-transport-terms-description` | 4053 | Delivery or transport terms description code |
| `DeliveryOrTransportTermsFunction` | `delivery-or-transport-terms-function` | 4055 | Delivery or transport terms function code |
| `ClauseQualifier` | `clause-qualifier` | 4059 | Clause code qualifier |
| `ContractAndCarriageCondition` | `contract-and-carriage-condition` | 4065 | Contract and carriage condition code |
| `ProvisoQualifier` | `proviso-qualifier` | 4071 | Proviso code qualifier |
| `HandlingInstructionDescription` | `handling-instruction-description` | 4079 | Handling instruction description code |
| `InformationDetailsQualifier` | `information-details-qualifier` | 4153 | Information details code qualifier |
| `SpecialCondition` | `special-condition` | 4183 | Special condition code |
| `TransportChargesPaymentMethod` | `transport-charges-payment-method` | 4215 | Transport charges payment method code |
| `TransportServicePriority` | `transport-service-priority` | 4219 | Transport service priority code |
| `DiscrepancyNatureIdentification` | `discrepancy-nature-identification` | 4221 | Discrepancy nature identification code |
| `MarkingInstructions` | `marking-instructions` | 4233 | Marking instructions code |
| `PaymentArrangement` | `payment-arrangement` | 4237 | Payment arrangement code |
| `PaymentTermsDescriptionIdentifier` | `payment-terms-description-identifier` | 4277 | Payment terms description identifier |
| `PaymentTermsTypeQualifier` | `payment-terms-type-qualifier` | 4279 | Payment terms type code qualifier |
| `ChangeReasonDescription` | `change-reason-description` | 4295 | Change reason description code |
| `ResponseType` | `response-type` | 4343 | Response type code |
| `ProductIdentifierQualifier` | `product-identifier-qualifier` | 4347 | Product identifier code qualifier |
| `BankOperation` | `bank-operation` | 4383 | Bank operation code |
| `InstructionDescription` | `instruction-description` | 4401 | Instruction description code |
| `InstructionTypeQualifier` | `instruction-type-qualifier` | 4403 | Instruction type code qualifier |
| `StatusDescription` | `status-description` | 4405 | Status description code |
| `SampleProcessStep` | `sample-process-step` | 4407 | Sample process step code |
| `TestAdministrationMethod` | `test-administration-method` | 4419 | Test administration method code |
| `PaymentGuaranteeMeans` | `payment-guarantee-means` | 4431 | Payment guarantee means code |
| `PaymentChannel` | `payment-channel` | 4435 | Payment channel code |
| `AccountTypeQualifier` | `account-type-qualifier` | 4437 | Account type code qualifier |
| `PaymentConditions` | `payment-conditions` | 4439 | Payment conditions code |
| `FreeTextFormat` | `free-text-format` | 4447 | Free text format code |
| `TextSubjectQualifier` | `text-subject-qualifier` | 4451 | Text subject code qualifier |
| `FreeTextFunction` | `free-text-function` | 4453 | Free text function code |
| `BackOrderArrangementType` | `back-order-arrangement-type` | 4455 | Back order arrangement type code |
| `SubstitutionCondition` | `substitution-condition` | 4457 | Substitution condition code |
| `PaymentMeans` | `payment-means` | 4461 | Payment means code |
| `IntraCompanyPaymentIndicator` | `intra-company-payment-indicator` | 4463 | Intra-company payment indicator code |
| `AdjustmentReasonDescription` | `adjustment-reason-description` | 4465 | Adjustment reason description code |
| `SettlementMeans` | `settlement-means` | 4471 | Settlement means code |
| `AccountingEntryTypeName` | `accounting-entry-type-name` | 4475 | Accounting entry type name code |
| `FinancialTransactionType` | `financial-transaction-type` | 4487 | Financial transaction type code |
| `DeliveryInstruction` | `delivery-instruction` | 4493 | Delivery instruction code |
| `InventoryMovementReason` | `inventory-movement-reason` | 4499 | Inventory movement reason code |
| `InventoryMovementDirection` | `inventory-movement-direction` | 4501 | Inventory movement direction code |
| `InventoryBalanceMethod` | `inventory-balance-method` | 4503 | Inventory balance method code |
| `CreditCoverRequestType` | `credit-cover-request-type` | 4505 | Credit cover request type code |
| `CreditCoverResponseType` | `credit-cover-response-type` | 4507 | Credit cover response type code |
| `CreditCoverResponseReason` | `credit-cover-response-reason` | 4509 | Credit cover response reason code |
| `RequestedInformationDescription` | `requested-information-description` | 4511 | Requested information description code |
| `MaintenanceOperation` | `maintenance-operation` | 4513 | Maintenance operation code |
| `SealCondition` | `seal-condition` | 4517 | Seal condition code |
| `SealType` | `seal-type` | 4525 | Seal type code |
| `MonetaryAmountFunctionDescription` | `monetary-amount-function-description` | 5007 | Monetary amount function description code |
| `IndexQualifier` | `index-qualifier` | 5013 | Index code qualifier |
| `MonetaryAmountTypeQualifier` | `monetary-amount-type-qualifier` | 5025 | Monetary amount type code qualifier |
| `IndexTypeIdentifier` | `index-type-identifier` | 5027 | Index type identifier |
| `IndexRepresentation` | `index-representation` | 5039 | Index representation code |
| `ContributionQualifier` | `contribution-qualifier` | 5047 | Contribution code qualifier |
| `ContributionTypeDescription` | `contribution-type-description` | 5049 | Contribution type description code |
| `PriceQualifier` | `price-qualifier` | 5125 | Price code qualifier |
| `DutyOrTaxOrFeeTypeName` | `duty-or-tax-or-fee-type-name` | 5153 | Duty or tax or fee type name code |
| `AllowanceOrChargeIdentification` | `allowance-or-charge-identification` | 5189 | Allowance or charge identification code |
| `SubLineItemPriceChangeOperation` | `sub-line-item-price-change-operation` | 5213 | Sub-line item price change operation code |
| `ChargeCategory` | `charge-category` | 5237 | Charge category code |
| `RateOrTariffClassDescription` | `rate-or-tariff-class-description` | 5243 | Rate or tariff class description code |
| `PercentageTypeQualifier` | `percentage-type-qualifier` | 5245 | Percentage type code qualifier |
| `PercentageBasisIdentification` | `percentage-basis-identification` | 5249 | Percentage basis identification code |
| `ChargeUnit` | `charge-unit` | 5261 | Charge unit code |
| `ServiceType` | `service-type` | 5267 | Service type code |
| `DutyOrTaxOrFeeRateBasis` | `duty-or-tax-or-fee-rate-basis` | 5273 | Duty or tax or fee rate basis code |
| `DutyOrTaxOrFeeFunctionQualifier` | `duty-or-tax-or-fee-function-qualifier` | 5283 | Duty or tax or fee function code qualifier |
| `DutyOrTaxOrFeeCategory` | `duty-or-tax-or-fee-category` | 5305 | Duty or tax or fee category code |
| `TaxOrDutyOrFeePaymentDueDate` | `tax-or-duty-or-fee-payment-due-date` | 5307 | Tax or duty or fee payment due date code |
| `RemunerationTypeName` | `remuneration-type-name` | 5315 | Remuneration type name code |
| `PriceType` | `price-type` | 5375 | Price type code |
| `ProductGroupType` | `product-group-type` | 5379 | Product group type code |
| `PriceSpecification` | `price-specification` | 5387 | Price specification code |
| `PriceMultiplierTypeQualifier` | `price-multiplier-type-qualifier` | 5393 | Price multiplier type code qualifier |
| `RateTypeQualifier` | `rate-type-qualifier` | 5419 | Rate type code qualifier |
| `AllowanceOrChargeQualifier` | `allowance-or-charge-qualifier` | 5463 | Allowance or charge code qualifier |
| `SubLineIndicator` | `sub-line-indicator` | 5495 | Sub-line indicator code |
| `RatePlan` | `rate-plan` | 5501 | Rate plan code |
| `GeographicalPositionQualifier` | `geographical-position-qualifier` | 6029 | Geographical position code qualifier |
| `QuantityTypeQualifier` | `quantity-type-qualifier` | 6063 | Quantity type code qualifier |
| `ControlTotalTypeQualifier` | `control-total-type-qualifier` | 6069 | Control total type code qualifier |
| `FrequencyQualifier` | `frequency-qualifier` | 6071 | Frequency code qualifier |
| `ResultRepresentation` | `result-representation` | 6077 | Result representation code |
| `ResultNormalcy` | `result-normalcy` | 6079 | Result normalcy code |
| `DosageAdministrationQualifier` | `dosage-administration-qualifier` | 6085 | Dosage administration code qualifier |
| `ResultValueTypeQualifier` | `result-value-type-qualifier` | 6087 | Result value type code qualifier |
| `LengthType` | `length-type` | 6113 | Length type code |
| `DimensionTypeQualifier` | `dimension-type-qualifier` | 6145 | Dimension type code qualifier |
| `NonDiscreteMeasurementName` | `non-discrete-measurement-name` | 6155 | Non-discrete measurement name code |
| `RangeTypeQualifier` | `range-type-qualifier` | 6167 | Range type code qualifier |
| `SizeTypeQualifier` | `size-type-qualifier` | 6173 | Size type code qualifier |
| `TemperatureTypeQualifier` | `temperature-type-qualifier` | 6245 | Temperature type code qualifier |
| `MeasurementPurposeQualifier` | `measurement-purpose-qualifier` | 6311 | Measurement purpose code qualifier |
| `MeasuredAttribute` | `measured-attribute` | 6313 | Measured attribute code |
| `MeasurementSignificance` | `measurement-significance` | 6321 | Measurement significance code |
| `StatisticTypeQualifier` | `statistic-type-qualifier` | 6331 | Statistic type code qualifier |
| `ExchangeRateCurrencyMarketIdentifier` | `exchange-rate-currency-market-identifier` | 6341 | Exchange rate currency market identifier |
| `CurrencyTypeQualifier` | `currency-type-qualifier` | 6343 | Currency type code qualifier |
| `CurrencyUsageQualifier` | `currency-usage-qualifier` | 6347 | Currency usage code qualifier |
| `UnitTypeQualifier` | `unit-type-qualifier` | 6353 | Unit type code qualifier |
| `MeasurementUnit` | `measurement-unit` | 6411 | Measurement unit code |
| `ClinicalInformationTypeQualifier` | `clinical-information-type-qualifier` | 6415 | Clinical information type code qualifier |
| `PhysicalOrLogicalStateTypeQualifier` | `physical-or-logical-state-type-qualifier` | 7001 | Physical or logical state type code qualifier |
| `PhysicalOrLogicalStateDescription` | `physical-or-logical-state-description` | 7007 | Physical or logical state description code |
| `ItemDescription` | `item-description` | 7009 | Item description code |
| `ItemAvailability` | `item-availability` | 7011 | Item availability code |
| `SampleSelectionMethod` | `sample-selection-method` | 7039 | Sample selection method code |
| `PowerType` | `power-type` | 7041 | Power type code |
| `SampleState` | `sample-state` | 7045 | Sample state code |
| `SampleDirection` | `sample-direction` | 7047 | Sample direction code |
| `ClassType` | `class-type` | 7059 | Class type code |
| `PackagingTermsAndConditions` | `packaging-terms-and-conditions` | 7073 | Packaging terms and conditions code |
| `PackagingLevel` | `packaging-level` | 7075 | Packaging level code |
| `DescriptionFormat` | `description-format` | 7077 | Description format code |
| `ItemCharacteristic` | `item-characteristic` | 7081 | Item characteristic code |
| `ConfigurationOperation` | `configuration-operation` | 7083 | Configuration operation code |
| `CargoTypeClassification` | `cargo-type-classification` | 7085 | Cargo type classification code |
| `CharacteristicValueDescription` | `characteristic-value-description` | 7111 | Characteristic value description code |
| `ProductDetailsTypeQualifier` | `product-details-type-qualifier` | 7133 | Product details type code qualifier |
| `ProductCharacteristicIdentification` | `product-characteristic-identification` | 7139 | Product characteristic identification code |
| `ItemTypeIdentification` | `item-type-identification` | 7143 | Item type identification code |
| `SpecialServiceDescription` | `special-service-description` | 7161 | Special service description code |
| `HierarchicalStructureRelationship` | `hierarchical-structure-relationship` | 7171 | Hierarchical structure relationship code |
| `HierarchyObjectQualifier` | `hierarchy-object-qualifier` | 7173 | Hierarchy object code qualifier |
| `ProcessTypeDescription` | `process-type-description` | 7187 | Process type description code |
| `PackagingRelatedDescription` | `packaging-related-description` | 7233 | Packaging related description code |
| `ServiceRequirement` | `service-requirement` | 7273 | Service requirement code |
| `SectorAreaIdentificationQualifier` | `sector-area-identification-qualifier` | 7293 | Sector area identification code qualifier |
| `RequirementOrConditionDescriptionIdentifier` | `requirement-or-condition-description-identifier` | 7295 | Requirement or condition description identifier |
| `SetTypeQualifier` | `set-type-qualifier` | 7297 | Set type code qualifier |
| `RequirementDesignator` | `requirement-designator` | 7299 | Requirement designator code |
| `ProcessingIndicatorDescription` | `processing-indicator-description` | 7365 | Processing indicator description code |
| `SurfaceOrLayer` | `surface-or-layer` | 7383 | Surface or layer code |
| `ObjectIdentificationQualifier` | `object-identification-qualifier` | 7405 | Object identification code qualifier |
| `IndexingStructureQualifier` | `indexing-structure-qualifier` | 7429 | Indexing structure code qualifier |
| `AgreementTypeQualifier` | `agreement-type-qualifier` | 7431 | Agreement type code qualifier |
| `AgreementTypeDescription` | `agreement-type-description` | 7433 | Agreement type description code |
| `MembershipTypeQualifier` | `membership-type-qualifier` | 7449 | Membership type code qualifier |
| `MembershipCategoryDescription` | `membership-category-description` | 7451 | Membership category description code |
| `MembershipLevelQualifier` | `membership-level-qualifier` | 7455 | Membership level code qualifier |
| `AttendeeCategoryDescription` | `attendee-category-description` | 7459 | Attendee category description code |
| `InventoryType` | `inventory-type` | 7491 | Inventory type code |
| `DamageDetailsQualifier` | `damage-details-qualifier` | 7493 | Damage details code qualifier |
| `ObjectTypeQualifier` | `object-type-qualifier` | 7495 | Object type code qualifier |
| `StructureComponentFunctionQualifier` | `structure-component-function-qualifier` | 7497 | Structure component function code qualifier |
| `MarkingType` | `marking-type` | 7511 | Marking type code |
| `StructureType` | `structure-type` | 7515 | Structure type code |
| `TrafficRestriction` | `traffic-restriction` | 8015 | Traffic restriction code |
| `ConveyanceCallPurposeDescription` | `conveyance-call-purpose-description` | 8025 | Conveyance call purpose description code |
| `TrafficRestrictionTypeQualifier` | `traffic-restriction-type-qualifier` | 8035 | Traffic restriction type code qualifier |
| `TransportStageQualifier` | `transport-stage-qualifier` | 8051 | Transport stage code qualifier |
| `EquipmentTypeQualifier` | `equipment-type-qualifier` | 8053 | Equipment type code qualifier |
| `EquipmentSupplier` | `equipment-supplier` | 8077 | Equipment supplier code |
| `TransitDirectionIndicator` | `transit-direction-indicator` | 8101 | Transit direction indicator code |
| `EquipmentSizeAndTypeDescription` | `equipment-size-and-type-description` | 8155 | Equipment size and type description code |
| `FullOrEmptyIndicator` | `full-or-empty-indicator` | 8169 | Full or empty indicator code |
| `TransportMeansDescription` | `transport-means-description` | 8179 | Transport means description code |
| `EquipmentStatus` | `equipment-status` | 8249 | Equipment status code |
| `DangerousGoodsRegulations` | `dangerous-goods-regulations` | 8273 | Dangerous goods regulations code |
| `TransportMeansOwnershipIndicator` | `transport-means-ownership-indicator` | 8281 | Transport means ownership indicator code |
| `TransportMovement` | `transport-movement` | 8323 | Transport movement code |
| `MovementTypeDescription` | `movement-type-description` | 8335 | Movement type description code |
| `PackagingDangerLevel` | `packaging-danger-level` | 8339 | Packaging danger level code |
| `HaulageArrangements` | `haulage-arrangements` | 8341 | Haulage arrangements code |
| `ReturnablePackageLoadContents` | `returnable-package-load-contents` | 8393 | Returnable package load contents code |
| `ReturnablePackageFreightPaymentResponsibility` | `returnable-package-freight-payment-responsibility` | 8395 | Returnable package freight payment responsibility code |
| `ExcessTransportationReason` | `excess-transportation-reason` | 8457 | Excess transportation reason code |
| `ExcessTransportationResponsibility` | `excess-transportation-responsibility` | 8459 | Excess transportation responsibility code |
| `EmploymentDetailsQualifier` | `employment-details-qualifier` | 9003 | Employment details code qualifier |
| `StatusReasonDescription` | `status-reason-description` | 9013 | Status reason description code |
| `StatusCategory` | `status-category` | 9015 | Status category code |
| `AttributeFunctionQualifier` | `attribute-function-qualifier` | 9017 | Attribute function code qualifier |
| `DefinitionFunction` | `definition-function` | 9023 | Definition function code |
| `DefinitionExtent` | `definition-extent` | 9025 | Definition extent code |
| `ValueDefinitionQualifier` | `value-definition-qualifier` | 9029 | Value definition code qualifier |
| `EditMaskRepresentation` | `edit-mask-representation` | 9031 | Edit mask representation code |
| `QualificationApplicationArea` | `qualification-application-area` | 9035 | Qualification application area code |
| `QualificationTypeQualifier` | `qualification-type-qualifier` | 9037 | Qualification type code qualifier |
| `FacilityTypeDescription` | `facility-type-description` | 9039 | Facility type description code |
| `ReservationIdentifierQualifier` | `reservation-identifier-qualifier` | 9043 | Reservation identifier code qualifier |
| `BasisQualifier` | `basis-qualifier` | 9045 | Basis code qualifier |
| `ApplicabilityQualifier` | `applicability-qualifier` | 9051 | Applicability code qualifier |
| `RelationshipTypeQualifier` | `relationship-type-qualifier` | 9141 | Relationship type code qualifier |
| `RelationshipDescription` | `relationship-description` | 9143 | Relationship description code |
| `SimpleDataElementCharacterRepresentation` | `simple-data-element-character-representation` | 9153 | Simple data element character representation code |
| `CodeSetIndicator` | `code-set-indicator` | 9161 | Code set indicator code |
| `DataRepresentationType` | `data-representation-type` | 9169 | Data representation type code |
| `DataElementUsageType` | `data-element-usage-type` | 9175 | Data element usage type code |
| `DutyRegimeType` | `duty-regime-type` | 9213 | Duty regime type code |
| `ValidationCriteria` | `validation-criteria` | 9285 | Validation criteria code |
| `SealingPartyName` | `sealing-party-name` | 9303 | Sealing party name code |
| `GovernmentProcedure` | `government-procedure` | 9353 | Government procedure code |
| `GovernmentInvolvement` | `government-involvement` | 9411 | Government involvement code |
| `GovernmentAgencyIdentification` | `government-agency-identification` | 9415 | Government agency identification code |
| `GovernmentAction` | `government-action` | 9417 | Government action code |
| `ProcessStageQualifier` | `process-stage-qualifier` | 9421 | Process stage code qualifier |
| `ClinicalInterventionDescription` | `clinical-intervention-description` | 9437 | Clinical intervention description code |
| `ClinicalInterventionTypeQualifier` | `clinical-intervention-type-qualifier` | 9441 | Clinical intervention type code qualifier |
| `AttendanceTypeQualifier` | `attendance-type-qualifier` | 9443 | Attendance type code qualifier |
| `DischargeTypeDescription` | `discharge-type-description` | 9447 | Discharge type description code |
| `CodeValueSource` | `code-value-source` | 9453 | Code value source code |
| `FormulaTypeQualifier` | `formula-type-qualifier` | 9501 | Formula type code qualifier |
| `FormulaSequenceQualifier` | `formula-sequence-qualifier` | 9507 | Formula sequence code qualifier |
| `FormulaSequenceOperand` | `formula-sequence-operand` | 9509 | Formula sequence operand code |
| `InformationCategory` | `information-category` | 9601 | Information category code |
| `DiagnosisType` | `diagnosis-type` | 9623 | Diagnosis type code |
| `RelatedCause` | `related-cause` | 9625 | Related cause code |
| `EventDetailsQualifier` | `event-details-qualifier` | 9635 | Event details code qualifier |
| `ServiceBasisQualifier` | `service-basis-qualifier` | 9641 | Service basis code qualifier |
| `SupportingEvidenceTypeQualifier` | `supporting-evidence-type-qualifier` | 9643 | Supporting evidence type code qualifier |
| `PayerResponsibilityLevel` | `payer-responsibility-level` | 9645 | Payer responsibility level code |
| `ProcessingInformationQualifier` | `processing-information-qualifier` | 9649 | Processing information code qualifier |

## Other features

* `serde` serializes a code as its string and deserializes it back.
* `case-insensitive` adds `from_code_ignore_case` and makes parsing case-insensitive.

## Usage

```rust
use untdid::{DocumentName, UnknownCode};

let invoice = DocumentName::from_code("380").expect("valid code");

assert_eq!(invoice, DocumentName::CommercialInvoice);
assert_eq!(invoice.code(), "380");
assert_eq!(invoice.name(), "Commercial invoice");
assert_eq!(invoice.to_string(), "380");

assert_eq!("380".parse::<DocumentName>(), Ok(invoice));
assert_eq!("ZZZZ".parse::<DocumentName>(), Err(UnknownCode("ZZZZ".to_owned())));

assert!(DocumentName::ALL.contains(&invoice));
```

A code list provides:

* `code()`, `name()`, and `description()` to read the code, its English name, and its description in the directory.
* `info()` to read all of them at once as a `CodeInfo`.
* `from_code()` to look up a code (case-sensitive).
* `from_code_ignore_case()` to look up a code ignoring the case (with the `case-insensitive` feature).
* `ALL` to iterate over every code in the order of the directory.
* `Display`, `FromStr`, and `TryFrom<&str>` implementations.

## Crates

* `untdid` provides the code list types.
* `untdid-macros` provides the procedural macro that generates the types from the code list file.

## Development

The workspace uses [cargo-make] and [cargo-hack]. The `check` task verifies formatting and runs clippy and tests for every feature alone and for all features together.

```sh
cargo make check
```

The `generated-tests` feature adds a test for every generated code.

## Minimum supported Rust version

Rust 1.85 (edition 2024).

## License

The source code of both crates is licensed under the [MIT license].

The `untdid` crate also contains the code list file `UNCL.24A` of the UN/EDIFACT directory D.24A. © United Nations.

The crate includes the original file without changes. The generated types contain modified data. The codes marked for deletion are skipped, the codes are split into one type per data element, and their names are converted into Rust identifiers.

### UN/CEFACT disclaimer

The code lists come from the UN/EDIFACT directory D.24A, an output of UN/CEFACT.

> UN/CEFACT draws attention to the possibility that the practice or implementation of its outputs (which include but are not limited to Recommendations, norms, standards, guidelines and technical specifications) may involve the use of a claimed intellectual property right.
>
> Each output is based on the contributions of participants in the UN/CEFACT process, who have agreed to waive enforcement of their intellectual property rights pursuant to the UN/CEFACT IPR Policy (document ECE/TRADE/CEFACT/2006/11 available at <http://www.unece.org/cefact/> or from the UNECE secretariat). UN/CEFACT takes no position concerning the evidence, validity or applicability of any claimed intellectual property right or any other right that might be claimed by any third parties related to the implementation of its outputs. UN/CEFACT makes no representation that it has made any investigation or effort to evaluate any such rights.
>
> Implementers of UN/CEFACT outputs are cautioned that any third party intellectual property rights claims related to their use of a UN/CEFACT output will be their responsibility and are urged to ensure that their use of UN/CEFACT outputs does not infringe on an intellectual property right of a third party.
>
> UN/CEFACT does not accept any liability for any possible infringement of a claimed intellectual property right or any other right that might be claimed to relate to the implementation of any of its outputs.

[MIT license]: https://opensource.org/licenses/MIT
[cargo-hack]: https://github.com/taiki-e/cargo-hack
[cargo-make]: https://github.com/sagiegurari/cargo-make
