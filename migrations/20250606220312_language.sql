-- Add migration script here

-- Create enum for framework type
CREATE TYPE framework_type AS ENUM ('client', 'server');

-- Create programming languages table
CREATE TABLE programming_languages (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create frameworks table
CREATE TABLE frameworks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    language_id INTEGER NOT NULL REFERENCES programming_languages(id),
    type framework_type NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, language_id)
);

-- Add some initial programming languages
