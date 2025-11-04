-- Add VPS management table

CREATE TABLE IF NOT EXISTS vps (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    hetzner_id BIGINT UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'stopped',
    server_type VARCHAR(100) NOT NULL,
    location VARCHAR(100) NOT NULL,
    image VARCHAR(255) NOT NULL,
    ipv4 VARCHAR(45),
    ipv6 VARCHAR(100),
    cpu_cores INTEGER NOT NULL,
    ram_gb INTEGER NOT NULL,
    disk_gb INTEGER NOT NULL,
    monthly_cost DECIMAL(10, 2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_vps_user_id ON vps(user_id);
CREATE INDEX idx_vps_hetzner_id ON vps(hetzner_id);
CREATE INDEX idx_vps_status ON vps(status);

-- Add VPS snapshots table
CREATE TABLE IF NOT EXISTS vps_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vps_id UUID NOT NULL REFERENCES vps(id) ON DELETE CASCADE,
    hetzner_snapshot_id BIGINT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    size_gb INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vps_snapshots_vps_id ON vps_snapshots(vps_id);

-- Add VPS backups table
CREATE TABLE IF NOT EXISTS vps_backups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vps_id UUID NOT NULL REFERENCES vps(id) ON DELETE CASCADE,
    hetzner_backup_id BIGINT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    size_gb INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vps_backups_vps_id ON vps_backups(vps_id);
