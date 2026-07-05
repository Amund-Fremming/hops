-- Add migration script here
CREATE TABLE "user" (
    "id" UUID PRIMARY KEY,
    "phone" VARCHAR(20),
    "phone_verified" BOOLEAN NOT NULL DEFAULT FALSE,
    "given_name" VARCHAR(100) NOT NULL,
    "family_name" VARCHAR(100) NOT NULL
);

CREATE INDEX idx_user_phone ON "user" ("phone");