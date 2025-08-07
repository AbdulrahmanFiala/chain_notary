# Category-Specific Metadata Implementation Guide

This document explains how to use the category-specific metadata approach for different document types in the Chain Notary system.

## Overview

The system now supports different document categories with their own specific metadata structures. This is implemented using:

1. **CollectionCategory** enum - defines available document categories
2. **CategorySpecificMetadata** enum - holds category-specific data
3. **DocumentMetadata** - enhanced with optional category-specific metadata

## Current Categories

### UniversityGraduationCertificate
- **Structure**: `Certificate` with `CertificateMetadata`
- **Fields**: issuer, recipient, title, description, issued date, expiry date, etc.
- **Features**: revocation support, additional metadata

## Usage Examples

### 1. Minting a Certificate Document

```rust
// Example: Mint a university graduation certificate
let certificate_data = CertificateData {
    name: "John Doe's Graduation Certificate".to_string(),
    description: Some("Bachelor of Science in Computer Science".to_string()),
    image_url: Some("https://example.com/certificate.jpg".to_string()),
    collection_id: Some("university_certs".to_string()),
    document_hash: "sha256:abc123...".to_string(),
    file_size: 1024,
    file_type: "application/pdf".to_string(),
    title: "Bachelor of Science in Computer Science".to_string(),
    document_type: DocumentType::GraduationCertificate,
    issued_date: 1640995200, // Unix timestamp
    expiry_date: None, // Certificates typically don't expire
    issuer_name: "University of Technology".to_string(),
    recipient_info: Some(RecipientInfo {
        name: "John Doe".to_string(),
        identifier: "STU123456".to_string(),
        contact: Some("john.doe@email.com".to_string()),
    }),
    additional_data: {
        let mut map = HashMap::new();
        map.insert("gpa".to_string(), "3.8".to_string());
        map.insert("major".to_string(), "Computer Science".to_string());
        map.insert("graduation_year".to_string(), "2024".to_string());
        map
    },
};

// Mint the certificate
mint_certificate("cert_001".to_string(), certificate_data)?;
```

### 2. Querying Certificate Data

```rust
// Get certificate data for a specific document
let certificate = get_certificate_data("cert_001".to_string())?;

// Check if certificate is revoked
let is_revoked = is_certificate_revoked("cert_001".to_string())?;

// Get all certificates by issuer
let issuer_certs = get_certificates_by_issuer(issuer_principal);

// Get certificates by recipient name
let recipient_certs = get_certificates_by_recipient("John Doe".to_string());

// Get all revoked certificates
let revoked_certs = get_revoked_certificates();
```

### 3. Certificate Management

```rust
// Revoke a certificate (only issuer can do this)
revoke_certificate("cert_001".to_string(), "Academic misconduct".to_string())?;

// Update certificate metadata (only issuer can do this)
let mut additional_data = HashMap::new();
additional_data.insert("updated_gpa".to_string(), "3.9".to_string());

update_certificate_metadata(
    "cert_001".to_string(),
    Some("Updated Certificate Title".to_string()),
    Some("Updated description".to_string()),
    Some(additional_data),
)?;
```

### 4. Working with Category-Specific Metadata

```rust
// Get document metadata
let doc_metadata = get_nft_metadata("cert_001".to_string())?;

// Check if it has category-specific metadata
if let Some(category) = doc_metadata.collection_category() {
    match category {
        CollectionCategory::UniversityGraduationCertificate => {
            // This is a certificate document
            if let Some(cert) = doc_metadata.as_certificate() {
                println!("Certificate: {}", cert.metadata.title);
                println!("Issuer: {}", cert.issuer_name);
                println!("Recipient: {}", cert.recipient_info.as_ref().unwrap().name);
            }
        }
    }
}
```

## Adding New Categories

To add a new document category:

1. **Add to CollectionCategory enum**:
```rust
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CollectionCategory {
    UniversityGraduationCertificate,
    ProfessionalLicense, // New category
    MedicalRecord,       // New category
}
```

2. **Create the specific metadata structure**:
```rust
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct License {
    pub license_id: String,
    pub license_type: String,
    pub issuing_authority: String,
    pub valid_from: u64,
    pub valid_until: u64,
    pub license_number: String,
    pub holder_name: String,
    pub is_suspended: bool,
    pub suspension_reason: Option<String>,
}
```

3. **Add to CategorySpecificMetadata enum**:
```rust
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CategorySpecificMetadata {
    UniversityGraduationCertificate(Certificate),
    ProfessionalLicense(License), // New variant
    MedicalRecord(MedicalRecord), // New variant
}
```

4. **Update helper methods**:
```rust
impl CategorySpecificMetadata {
    pub fn category(&self) -> CollectionCategory {
        match self {
            CategorySpecificMetadata::UniversityGraduationCertificate(_) => {
                CollectionCategory::UniversityGraduationCertificate
            }
            CategorySpecificMetadata::ProfessionalLicense(_) => {
                CollectionCategory::ProfessionalLicense
            }
            CategorySpecificMetadata::MedicalRecord(_) => {
                CollectionCategory::MedicalRecord
            }
        }
    }

    pub fn as_license(&self) -> Option<&License> {
        match self {
            CategorySpecificMetadata::ProfessionalLicense(license) => Some(license),
            _ => None,
        }
    }
}
```

5. **Add category-specific functions**:
```rust
#[update]
pub fn mint_license(token_id: String, license_data: LicenseData) -> MintResult {
    // Implementation similar to mint_certificate
}

#[query]
pub fn get_license_data(document_id: String) -> Option<License> {
    get_nft_metadata(document_id)?.as_license().cloned()
}
```

## Benefits of This Approach

1. **Type Safety**: Each category has its own strongly-typed structure
2. **Extensibility**: Easy to add new categories without breaking existing code
3. **Flexibility**: Different categories can have completely different metadata
4. **Backward Compatibility**: Existing documents without category metadata still work
5. **Query Efficiency**: Can filter and query by category
6. **Business Logic**: Category-specific operations (like revocation for certificates)

## Best Practices

1. **Always check category before accessing specific metadata**:
```rust
if let Some(cert) = doc_metadata.as_certificate() {
    // Work with certificate data
}
```

2. **Use helper methods for type-safe access**:
```rust
// Good
let category = doc_metadata.collection_category();

// Better - more specific
if let Some(cert) = doc_metadata.as_certificate() {
    // Work with certificate
}
```

3. **Validate category-specific data during minting**:
```rust
// Validate certificate data before minting
if certificate_data.issued_date > certificate_data.expiry_date.unwrap_or(u64::MAX) {
    return Err("Invalid date range".to_string());
}
```

4. **Use appropriate error handling**:
```rust
match get_certificate_data(document_id) {
    Some(cert) => {
        // Work with certificate
    }
    None => {
        // Handle case where document is not a certificate
    }
}
```

This approach provides a robust foundation for handling different document types while maintaining type safety and extensibility. 