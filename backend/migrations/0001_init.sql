CREATE TABLE devices (
    id INTEGER PRIMARY KEY,

    manufacturer TEXT NOT NULL,
    serial TEXT NOT NULL UNIQUE,
    capacity TEXT,

    assigned_number TEXT,
    registered BOOLEAN NOT NULL,
    secret BOOLEAN NOT NULL,
    special BOOLEAN NOT NULL,

    secclass TEXT,
    max_secclass TEXT,

    owner TEXT,

    register_number TEXT,
    conclusion_number TEXT,
    prescription TEXT,
    zones TEXT,

    destroyed BOOLEAN NOT NULL

);

CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
