-- Add down migration script here
ALTER TABLE clients DROP CONSTRAINT unique_username;
