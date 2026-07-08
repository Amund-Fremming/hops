-- Add migration script here
DROP TABLE IF EXISTS "audit_log";
DROP TABLE IF EXISTS "refresh_token";
DROP TABLE IF EXISTS "user_credential";
DROP TABLE IF EXISTS "user_identity";
DROP INDEX IF EXISTS idx_user_email;
DROP INDEX IF EXISTS idx_user_phone_number;
DROP TABLE IF EXISTS "user";
