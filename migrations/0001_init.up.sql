-- Add migration script here
CREATE TABLE "user" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "phone_number" VARCHAR(20),
    "phone_number_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "email" VARCHAR(255),
    "email_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "given_name" VARCHAR(100) NOT NULL,
    "family_name" VARCHAR(100) NOT NULL,
    "avatar_url" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_phone_number ON "user" ("phone_number");
CREATE INDEX idx_user_email ON "user" ("email");

CREATE TABLE "user_identity" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "provider_type" VARCHAR(50) NOT NULL,
    "provider_id" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE ("provider_type", "provider_id")
);

CREATE INDEX idx_user_identity_user_id ON "user_identity" ("user_id");

CREATE TABLE "user_credential" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "identity_id" UUID NOT NULL UNIQUE REFERENCES "user_identity"("id") ON DELETE CASCADE,
    "password_hash" TEXT NOT NULL,
    "algorithm" VARCHAR(50) NOT NULL DEFAULT 'argon2id',
    "failed_attempts" INT NOT NULL DEFAULT 0,
    "locked_until" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE "session" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL REFERENCES "user"("id") ON DELETE CASCADE,
    "refresh_token_hash" VARCHAR(64) NOT NULL UNIQUE,
    "user_agent" TEXT,
    "device_id" UUID NOT NULL UNIQUE,
    "device_name" VARCHAR(50) NOT NULL,
    "expires_at" TIMESTAMPTZ NOT NULL,
    "revoked_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "last_used_at" TIMESTAMPTZ
);

CREATE INDEX idx_session_user_id ON "session" ("user_id");
CREATE INDEX idx_session_device_token ON "session" ("device_id", "refresh_token_hash");

CREATE TYPE resource_type AS ENUM ('user');
CREATE TYPE action AS ENUM ('login_success', 'login_failed', 'account_locked', 'password_change');

CREATE TABLE "audit_log" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "resource_id" UUID NOT NULL,
    "resource_type" resource_type NOT NULL,
    "action" "action" NOT NULL,
    "ip_address" VARCHAR(45),
    "user_agent" TEXT,
    "metadata" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_resource ON "audit_log" ("resource_type", "resource_id");
CREATE INDEX idx_audit_log_created_at ON "audit_log" ("created_at");

CREATE TABLE "otp" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "phone_number" VARCHAR(20) NOT NULL,
    "hash" VARCHAR(128) NOT NULL,
    "expires_at" TIMESTAMPTZ NOT NULL,
    "verified_at" TIMESTAMPTZ DEFAULT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "ip_address" VARCHAR(45),
    "failed_attempts" INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_otp_phone_number_created ON "otp" ("phone_number", "created_at");