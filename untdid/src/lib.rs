//! UN/EDIFACT code lists of the United Nations Trade Data Interchange Directory (UNTDID),
//! release D.24A.
//!
//! All data is generated at compile time from the official code list file `UNCL.24A`
//! and embedded as `static` data. No runtime allocations, no I/O.
//! Every data element gives a type of its own behind a feature named after the element,
//! so a build contains only the code lists it enables. The codes the directory marks
//! for deletion are skipped.
//!
//! # Example
//!
//! ```rust
//! # #[cfg(feature = "document-name")]
//! # {
//! use untdid::DocumentName;
//!
//! let invoice = DocumentName::from_code("380").unwrap();
//! assert_eq!(invoice, DocumentName::CommercialInvoice);
//! assert_eq!(invoice.name(), "Commercial invoice");
//! assert_eq!(invoice.to_string(), "380");
//! # }
//! ```
//!
//! # License
//!
//! The source code is licensed under the [MIT license].
//!
//! The crate also contains the code list file `UNCL.24A`
//! of the UN/EDIFACT directory D.24A. © United Nations.
//!
//! The crate includes the original file without changes.
//! The generated types contain modified data. The codes marked for deletion are skipped,
//! the codes are split into one type per data element,
//! and their names are converted into Rust identifiers.
//!
//! ## UN/CEFACT disclaimer
//!
//! The code lists come from the UN/EDIFACT directory D.24A, an output of UN/CEFACT.
//!
//! > UN/CEFACT draws attention to the possibility that the practice or implementation of its
//! > outputs (which include but are not limited to Recommendations, norms, standards, guidelines
//! > and technical specifications) may involve the use of a claimed intellectual property right.
//! >
//! > Each output is based on the contributions of participants in the UN/CEFACT process,
//! > who have agreed to waive enforcement of their intellectual property rights
//! > pursuant to the UN/CEFACT IPR Policy (document ECE/TRADE/CEFACT/2006/11 available at
//! > <http://www.unece.org/cefact/> or from the UNECE secretariat).
//! > UN/CEFACT takes no position concerning the evidence, validity or applicability of any
//! > claimed intellectual property right or any other right that might be claimed
//! > by any third parties related to the implementation of its outputs.
//! > UN/CEFACT makes no representation that it has made any investigation
//! > or effort to evaluate any such rights.
//! >
//! > Implementers of UN/CEFACT outputs are cautioned that any third party
//! > intellectual property rights claims related to their use of a UN/CEFACT output
//! > will be their responsibility and are urged to ensure that their use of UN/CEFACT outputs
//! > does not infringe on an intellectual property right of a third party.
//! >
//! > UN/CEFACT does not accept any liability for any possible infringement
//! > of a claimed intellectual property right or any other right that might be claimed
//! > to relate to the implementation of any of its outputs.
//!
//! [MIT license]: https://opensource.org/licenses/MIT

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::too_many_lines, clippy::match_same_arms)]

mod prelude;

untdid_macros::untdid!("UNCL.24A");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "document-name")]
    #[test]
    fn looks_up_a_code_and_its_name() {
        let invoice = DocumentName::from_code("380").unwrap();

        assert_eq!(invoice, DocumentName::CommercialInvoice);
        assert_eq!(invoice.code(), "380");
        assert_eq!(invoice.name(), "Commercial invoice");
    }

    #[cfg(feature = "document-name")]
    #[test]
    fn keeps_the_description_of_a_code() {
        let invoice = DocumentName::CommercialInvoice;

        assert_eq!(
            invoice.description(),
            Some(
                "(1334) Document/message claiming payment for goods or services supplied \
                 under conditions agreed between seller and buyer."
            )
        );
    }

    #[cfg(feature = "document-name")]
    #[test]
    fn joins_a_name_wrapped_over_lines() {
        let instructions = DocumentName::from_code("48").unwrap();

        assert_eq!(
            instructions.name(),
            "Written instructions in conformance with ADR article number 10385"
        );
    }

    #[cfg(feature = "document-name")]
    #[test]
    fn joins_a_name_wrapped_after_a_hyphen() {
        let application = DocumentName::from_code("353").unwrap();

        assert!(application.name().ends_with("in night-time"));
    }

    #[cfg(feature = "document-name")]
    #[test]
    fn adds_the_code_to_the_variants_of_equal_names() {
        assert_eq!(
            DocumentName::from_code("479"),
            Some(DocumentName::EnvironmentalSocialAndGovernanceEsgReport479)
        );
        assert_eq!(
            DocumentName::from_code("939"),
            Some(DocumentName::EnvironmentalSocialAndGovernanceEsgReport939)
        );
    }

    #[cfg(feature = "despatch-pattern")]
    #[test]
    fn spells_out_a_leading_number_of_a_name() {
        assert_eq!(
            DespatchPattern::from_code("1"),
            Some(DespatchPattern::FirstWeekOfTheMonth)
        );
    }

    #[cfg(feature = "transport-means-description")]
    #[test]
    fn skips_a_deleted_code() {
        assert!(TransportMeansDescription::from_code("1").is_none());
    }

    #[cfg(feature = "document-name")]
    #[test]
    fn rejects_an_unknown_code() {
        assert_eq!(
            "ZZZZ".parse::<DocumentName>(),
            Err(UnknownCode("ZZZZ".to_owned()))
        );
    }

    #[cfg(all(feature = "reference-qualifier", feature = "case-insensitive"))]
    #[test]
    fn looks_up_a_code_ignoring_the_case() {
        let expected = ReferenceQualifier::from_code("AAA").unwrap();

        assert_eq!(
            ReferenceQualifier::from_code_ignore_case("aaa"),
            Some(expected)
        );
        assert_eq!("aaa".parse::<ReferenceQualifier>(), Ok(expected));
    }

    #[cfg(all(feature = "document-name", feature = "serde"))]
    #[test]
    fn serializes_a_code_as_its_string() {
        let invoice = DocumentName::CommercialInvoice;

        let encoded = serde_json::to_string(&invoice).unwrap();
        let decoded: DocumentName = serde_json::from_str(&encoded).unwrap();

        assert_eq!(encoded, "\"380\"");
        assert_eq!(decoded, invoice);
    }
}
