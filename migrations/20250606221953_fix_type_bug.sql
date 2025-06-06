-- Add migration script here

-- Rename the type column to f_type (first letter of frameworks)
ALTER TABLE frameworks RENAME COLUMN type TO f_type;
