-- Add migration script here
ALTER TABLE todos ADD COLUMN category VARCHAR(255) NOT NULL DEFAULT 'default';
