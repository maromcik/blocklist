CREATE TABLE IF NOT EXISTS "blocklist"
(
    id                  bigserial    PRIMARY KEY,
    ------------------------------  ---------------
    ip                  cidr         NOT NULL UNIQUE,
    version             smallint     NOT NULL,
    description         text
);

CREATE INDEX IF NOT EXISTS "blocklist_ip_version_idx" ON blocklist (version);