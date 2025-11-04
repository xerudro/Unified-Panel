-- Initial database schema for Unified Hosting Panel

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    company VARCHAR(255),
    timezone VARCHAR(100),
    avatar_url TEXT,
    mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_secret VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- Login attempts table
CREATE TABLE IF NOT EXISTS login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    attempted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_login_attempts_email ON login_attempts(email);
CREATE INDEX idx_login_attempts_ip ON login_attempts(ip_address);
CREATE INDEX idx_login_attempts_timestamp ON login_attempts(attempted_at);

-- Servers table
CREATE TABLE IF NOT EXISTS servers (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'offline',
    server_type VARCHAR(50) NOT NULL,
    location VARCHAR(100),
    cpu_cores INTEGER,
    ram_gb INTEGER,
    disk_gb INTEGER,
    os VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_servers_user_id ON servers(user_id);
CREATE INDEX idx_servers_status ON servers(status);
CREATE INDEX idx_servers_type ON servers(server_type);

-- Server metrics table
CREATE TABLE IF NOT EXISTS server_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    cpu_usage REAL NOT NULL,
    memory_usage REAL NOT NULL,
    disk_usage REAL NOT NULL,
    network_in BIGINT NOT NULL,
    network_out BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_server_metrics_server_id ON server_metrics(server_id);
CREATE INDEX idx_server_metrics_timestamp ON server_metrics(timestamp);

-- Websites table
CREATE TABLE IF NOT EXISTS websites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    server_id UUID REFERENCES servers(id) ON DELETE SET NULL,
    domain VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    application_type VARCHAR(50) NOT NULL,
    php_version VARCHAR(20),
    document_root VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_websites_user_id ON websites(user_id);
CREATE INDEX idx_websites_server_id ON websites(server_id);
CREATE INDEX idx_websites_domain ON websites(domain);

-- Domains table
CREATE TABLE IF NOT EXISTS domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    registrar VARCHAR(100),
    expires_at TIMESTAMP WITH TIME ZONE,
    auto_renew BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_domains_user_id ON domains(user_id);
CREATE INDEX idx_domains_name ON domains(name);
CREATE INDEX idx_domains_status ON domains(status);

-- DNS records table
CREATE TABLE IF NOT EXISTS dns_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain_id UUID NOT NULL REFERENCES domains(id) ON DELETE CASCADE,
    type VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    ttl INTEGER NOT NULL DEFAULT 3600,
    priority INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dns_records_domain_id ON dns_records(domain_id);
CREATE INDEX idx_dns_records_type ON dns_records(type);

-- Hosting packages table
CREATE TABLE IF NOT EXISTS hosting_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    storage_gb INTEGER NOT NULL,
    bandwidth_gb INTEGER NOT NULL,
    databases INTEGER NOT NULL,
    email_accounts INTEGER NOT NULL,
    price_monthly DECIMAL(10, 2) NOT NULL,
    price_yearly DECIMAL(10, 2) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Subscriptions table
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    package_id UUID NOT NULL REFERENCES hosting_packages(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    billing_cycle VARCHAR(20) NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    next_billing_date TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX idx_subscriptions_package_id ON subscriptions(package_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);

-- Logs table
CREATE TABLE IF NOT EXISTS logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    level VARCHAR(20) NOT NULL,
    category VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    server_id UUID REFERENCES servers(id) ON DELETE SET NULL,
    ip_address VARCHAR(45),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_logs_level ON logs(level);
CREATE INDEX idx_logs_category ON logs(category);
CREATE INDEX idx_logs_user_id ON logs(user_id);
CREATE INDEX idx_logs_server_id ON logs(server_id);
CREATE INDEX idx_logs_created_at ON logs(created_at);

-- Insert default admin user (password: admin123 - CHANGE THIS!)
INSERT INTO users (id, email, password_hash, role, created_at, updated_at, mfa_enabled)
VALUES (
    gen_random_uuid(),
    'admin@unified-panel.local',
    '$argon2id$v=19$m=19456,t=2,p=1$8X3FwVUxQDe8Kz5dVLsVxw$X7qJ5TKj9LZGqZrZDLWV5JxKHqLJQ5wZVU3f3qJ5TKk',
    'admin',
    NOW(),
    NOW(),
    false
);
