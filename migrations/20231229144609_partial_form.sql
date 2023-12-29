-- Add migration script here
CREATE TABLE partial_form (
    id UUID PRIMARY KEY NOT NULL,
    section INTEGER NOT NULL CHECK (section IN (1, 2, 3)),
    field JSON NOT NULL,
    complete BOOLEAN NOT NULL
);
