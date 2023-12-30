-- Add migration script here
ALTER TABLE partial_form
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now();
