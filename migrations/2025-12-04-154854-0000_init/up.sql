CREATE TABLE IF NOT EXISTS "blocklist"
(
    id                  bigserial    PRIMARY KEY,
    ------------------------------  ---------------
    ip                  cidr         NOT NULL UNIQUE,
    version             smallint     NOT NULL,
    country_code        text,
    isp                 text,
    user_agent          text,
    description         text
);

CREATE INDEX IF NOT EXISTS "blocklist_ip_version_idx" ON blocklist (version);