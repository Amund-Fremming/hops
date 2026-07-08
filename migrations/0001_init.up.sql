-- Add migration script here
CREATE TABLE "user" (
    "id" UUID PRIMARY KEY,
    "phone" VARCHAR(20),
    "phone_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "email" VARCHAR(255),
    "email_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "given_name" VARCHAR(100) NOT NULL,
    "family_name" VARCHAR(100) NOT NULL,
    "avatar_url" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_phone ON "user" ("phone");
CREATE INDEX idx_user_email ON "user" ("email");

CREATE TABLE "user_identity" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "provider_type" VARCHAR(50) NOT NULL,
    "provider_id" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE ("provider_type", "provider_id")
);

CREATE INDEX idx_user_identity_user_id ON "user_identity" ("user_id");

CREATE TABLE "user_credential" (
    "id" UUID PRIMARY KEY,
    "identity_id" UUID NOT NULL UNIQUE REFERENCES "user_identity"("id") ON DELETE CASCADE,
    "password_hash" TEXT NOT NULL,
    "algorithm" VARCHAR(50) NOT NULL DEFAULT 'argon2id',
    "failed_attempts" INT NOT NULL DEFAULT 0,
    "locked_until" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE "refresh_token" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "token_hash" VARCHAR(64) NOT NULL UNIQUE,
    "user_agent" TEXT,
    "device_id" VARCHAR(255),
    "expires_at" TIMESTAMPTZ NOT NULL,
    "revoked_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "last_used_at" TIMESTAMPTZ
);

CREATE INDEX idx_refresh_token_user_id ON "refresh_token" ("user_id");
CREATE INDEX idx_refresh_token_token_hash ON "refresh_token" ("token_hash");

CREATE TABLE "audit_log" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID REFERENCES "user"("id") ON DELETE SET NULL,
    "resource_id" UUID NOT NULL,
    "resource_type" VARCHAR(100) NOT NULL,
    "action" VARCHAR(100) NOT NULL,
    "ip_address" VARCHAR(45),
    "user_agent" TEXT,
    "metadata" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user_id ON "audit_log" ("user_id");
CREATE INDEX idx_audit_log_resource ON "audit_log" ("resource_type", "resource_id");
CREATE INDEX idx_audit_log_created_at ON "audit_log" ("created_at");