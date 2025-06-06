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
INSERT INTO programming_languages (name) VALUES
    ('JavaScript'),
    ('TypeScript'),
    ('Python'),
    ('Java'),
    ('C#'),
    ('PHP'),
    ('Ruby'),
    ('Go'),
    ('Rust');

-- Add some initial frameworks
INSERT INTO frameworks (name, language_id, type) VALUES
    ('React', 1, 'client'),
    ('Vue.js', 1, 'client'),
    ('Angular', 2, 'client'),
    ('Flask', 3, 'server'),
    ('Spring', 4, 'server'),
    ('ASP.NET', 5, 'server'),
    ('Symfony', 6, 'server'),
    ('Next.js', 2, 'server'),
    ('Axum', 9, 'server'),
    ('Lepton', 9, 'client');
