use core::fmt;

#[derive(Clone, Debug)]
pub enum BusinessError {
    DecryptionError(String),
    WebauthnError(String),
    EmailError(String),
    SmsError(String),
    DbError(DbErrors),
    JwtError(String),
    PaymentError(String),
    GenericError(String),
}

impl fmt::Display for BusinessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BusinessError::DecryptionError(s) => write!(f, "BusinessError: DecryptionError: Unable to decrypt: {}", s),
            BusinessError::WebauthnError(s) => write!(f, "BusinessError: WebauthnError: Unable to do webauthn: {}", s),
            BusinessError::EmailError(s) => write!(f, "BusinessError: EmailError: {}", s),
            BusinessError::SmsError(s) => write!(f, "BusinessError: SmsError: {}", s),
            BusinessError::DbError(e) => write!(f, "BusinessError: {}", e),
            BusinessError::JwtError(s) => write!(f, "BusinessError: JwtError: {}", s),
            BusinessError::PaymentError(s) => write!(f, "BusinessError: PaymentError: {}", s),
            BusinessError::GenericError(s) => write!(f, "BusinessError: {}", s),
        }
    }
}
#[derive(Clone, Debug)]
pub enum DbErrors {
    DbUnavailable,
    UserProvisionError,
    User(String),
    Credential(String),
    Regstate(String),
    OtpNotFound,
    Transaction(String),
    Arbitration(String),
    Payment(String),
    ArbitrationAcceptError,
    ArbitrationResolveError,
}

impl fmt::Display for DbErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbErrors::DbUnavailable => write!(f, "Cannot connect to the database"),
            DbErrors::UserProvisionError => write!(f, "Unable to prevision the user"),
            DbErrors::User(s) => write!(f, "User lookup issue: {}", s),
            DbErrors::Credential(s) => write!(f, "Credential lookup issue: {}", s),
            DbErrors::Regstate(s) => write!(f, "Regstate lookup issue: {}", s),
            DbErrors::OtpNotFound => write!(f, "Unable to find the One Time Token code for a user"),
            DbErrors::Transaction(s) => write!(f, "Transaction lookup issue: {}",s ),
            DbErrors::Arbitration(s) => write!(f, "Arbitration lookup issue: {}",s ),
            DbErrors::Payment(s) => write!(f, "Payment gateway issue: {}", s),
            DbErrors::ArbitrationAcceptError => write!(f, "Unable to accept arbitration to Database"),
            DbErrors::ArbitrationResolveError => write!(f, "Unable to resolve arbitration in Database"),
        }
    }
}