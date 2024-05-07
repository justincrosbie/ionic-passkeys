CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    mobile VARCHAR NOT NULL,
    public_key VARCHAR NOT NULL,
    user_state VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    modified_at TIMESTAMP,
    verified BOOLEAN NOT NULL
);

CREATE TABLE credentials (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL,
    credentialid VARCHAR NOT NULL,
    passkey TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    modified_at TIMESTAMP,
    FOREIGN KEY (user_uuid) REFERENCES users(uuid)
);

create table REGSTATES (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL,
    username VARCHAR NOT NULL,
    challenge VARCHAR UNIQUE NOT NULL,
    reg_state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    modified_at TIMESTAMP
);

-- INSERT INTO users VALUES ('f81b49ee-3537-4f37-bb23-0ce0228b270f', 'justincrosbie', 'justincrosbie@gmail.com', '01234567', 'Ap_mJ3GoR76Pg4Gy8cUEwU7RjIL6i8DG49EQRG3uePda', 'ADMIN', 'R3kJWUjz3RhS1zsNbAbxJV8r98qzU5pQHtTEHajVomWMi90j5FMq0jH-3KVHUjRdpUVrWHz20lb_9b9qxN5R6g==','EMAIL',0,'','',0,FALSE);

-- INSERT INTO properties VALUES ('9544939a-18e2-49b4-82cd-24be06dd0e18', 'email.smtp.host', 'SMTP Hostname', 'smtp.gmail.com', 'The hostname of SMTP server', 'text');
-- INSERT INTO properties VALUES ('0e1eeac3-9049-40ef-8094-ced552c876f1', 'email.smtp.port', 'SMTP Port', '587', 'The port of SMTP server', 'number');
-- INSERT INTO properties VALUES ('6eef9c2d-759c-409d-a4fd-b1c767b592cd', 'email.smtp.display', 'SMTP Display Name', 'Wallet Nation (no-reply)', 'The display name in the email reply address', 'text');
-- INSERT INTO properties VALUES ('93170739-4529-4648-973a-5e97db928a07', 'email.smtp.username', 'SMTP Username', 'noreplywalletnation@gmail.com', 'The username used to login to the SMTP server', 'text');
-- INSERT INTO properties VALUES ('6d03b9d0-7816-408c-b6b8-7d499266810f', 'email.smtp.tls', 'SMTP TLS Enabled', 'true', 'Email service accepts STARTTLS', 'checkbox');
-- INSERT INTO properties VALUES ('712d0b27-3138-46bd-b802-94a565086d68', 'email.smtp.password', 'SMTP Password', 'xljftawutsrwyidc', 'The password to login SMTP server', 'password');
-- INSERT INTO properties VALUES ('2b02295f-d460-49ce-b15f-96f2f59457f2', 'payment.paypal.client.id', 'Paypal Client ID', 'XXXXXX', 'The client id of the paypal account', 'text');
-- INSERT INTO properties VALUES ('257be71a-b134-4ba6-bcd4-df43bca0eef0', 'payment.paypal.client.secret', 'Paypal client secret', 'xxxxxx', 'The client secret for the paypal account', 'password');
-- INSERT INTO properties VALUES ('257be71a-b134-4ba6-bcd4-df43bca0eef1', 'payment.paypal.client.amount', 'Paypal amount', '10.0', 'The amount a customer will pay for the service', 'text');
-- INSERT INTO properties VALUES ('388b364c-bc1f-4ce6-8877-99d352cd2c56', 'sms.twilio.number', 'Twilio Phone number', 'xxxxxxx', 'The from mobile number', 'text');
-- INSERT INTO properties VALUES ('01551057-48e7-4fa8-abaa-825640735927', 'sms.twilio.sid', 'Twilio SID', 'xxxxxx', 'The registered SID with Twilio', 'text');
-- INSERT INTO properties VALUES ('4baccc87-1604-4ced-bb57-4e2503295e97', 'sms.twilio.api.key', 'Twilio API key', 'xxxxxx', 'The registered API key', 'text');
-- INSERT INTO properties VALUES ('5fdc4a9e-90cb-4a32-917c-a64f34fce012', 'sms.twilio.api.secret', 'Twilio API Secret', 'xxxxxxx', 'The registered API Secret', 'password');
-- INSERT INTO properties VALUES ('b85a6b21-c713-49ee-b38e-20c71dba0bee', 'audit.transaction.count', 'Number of transactions', '1', 'Transactions between sending audit information', 'number');
