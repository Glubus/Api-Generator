-- Add migration script here

-- Add codegenr_config column to frameworks table
ALTER TABLE frameworks ADD COLUMN codegenr_config text[];

-- Add codegenr_config column to programming_languages table

INSERT INTO programming_languages (name) VALUES
    ('TypeScript'),
    ('Python'),
    ('Rust');

-- Add some initial frameworks
INSERT INTO frameworks (name, language_id, f_type, codegenr_config) VALUES
    ('React', 1, 'client', ARRAY['client_services_ts', 'client_models_ts']),
    ('Flask', 2, 'server', ARRAY['server_models_flask', 'server_controller_flask', 'server_helpers_flask', 'server_app_flask']),
    ('Axum', 3, 'server', ARRAY['server_models_axum']);