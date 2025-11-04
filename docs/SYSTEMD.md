# Systemd Deployment Guide

This is the **primary and recommended** deployment method for Unified Hosting Panel. Systemd provides native process management, automatic restarts, resource limits, and seamless integration with your Linux system.

## Why Systemd?

- **Native Integration**: Built into all modern Linux distributions
- **Automatic Restarts**: Handles crashes and ensures high availability
- **Resource Management**: Control CPU, memory, and I/O limits
- **Logging**: Integrated with journald for centralized logging
- **Security**: Process isolation and privilege control
- **Dependency Management**: Automatic startup order
- **No Container Overhead**: Direct execution for maximum performance

## Prerequisites

- Linux server (Ubuntu 22.04 LTS or Debian 12 recommended)
- PostgreSQL 15+
- Rust 1.75+ (for building)
- Nginx (for reverse proxy and TLS)
- Systemd (installed by default)

## Installation Steps

### 1. System Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    postgresql \
    postgresql-contrib \
    nginx \
    git \
    curl
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Create System User

```bash
# Create dedicated user for security
sudo useradd -r -m -d /opt/unified-panel -s /bin/bash unified-panel

# Create necessary directories
sudo mkdir -p /opt/unified-panel/backend
sudo mkdir -p /etc/unified-panel
sudo mkdir -p /var/log/unified-panel

# Set ownership
sudo chown -R unified-panel:unified-panel /opt/unified-panel
sudo chown -R unified-panel:unified-panel /var/log/unified-panel
```

### 4. Database Setup

```bash
# Switch to postgres user
sudo -u postgres psql << EOF
-- Create database
CREATE DATABASE unified_panel;

-- Create user with secure password
CREATE USER panel_user WITH ENCRYPTED PASSWORD 'YOUR_SECURE_PASSWORD_HERE';

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE unified_panel TO panel_user;

-- Grant schema permissions (PostgreSQL 15+)
\c unified_panel
GRANT ALL ON SCHEMA public TO panel_user;
GRANT ALL ON ALL TABLES IN SCHEMA public TO panel_user;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO panel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO panel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO panel_user;
EOF
```

### 5. Clone and Build Application

```bash
# Switch to panel user
sudo -u unified-panel -i

# Clone repository
cd /opt/unified-panel
git clone https://github.com/xerudro/unified-panel.git .

# Enter backend directory
cd backend

# Create environment file
cp .env.example .env

# Edit configuration
nano .env
```

Configure your `.env`:

```env
# Server
HOST=0.0.0.0
PORT=3000
RUST_LOG=info

# Database
DATABASE_URL=postgresql://panel_user:YOUR_SECURE_PASSWORD_HERE@localhost/unified_panel

# Security (generate strong random strings!)
JWT_SECRET=$(openssl rand -base64 64)
SESSION_SECRET=$(openssl rand -base64 64)

# Hetzner Cloud (get from https://console.hetzner.cloud/)
HETZNER_API_TOKEN=your_hetzner_api_token_here

# CORS
CORS_ORIGIN=https://panel.yourdomain.com
```

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run

# Build release binary
cargo build --release

# Exit back to your admin user
exit
```

### 6. Install Systemd Service

```bash
# Copy service file
sudo cp /opt/unified-panel/unified-panel.service /etc/systemd/system/

# Edit service file with your actual values
sudo nano /etc/systemd/system/unified-panel.service

# Reload systemd
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable unified-panel

# Start service
sudo systemctl start unified-panel

# Check status
sudo systemctl status unified-panel
```

### 7. Configure Nginx Reverse Proxy

Create `/etc/nginx/sites-available/unified-panel`:

```nginx
# Upstream to Rust application
upstream unified_panel_backend {
    server 127.0.0.1:3000 fail_timeout=0;
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name panel.yourdomain.com;

    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    location / {
        return 301 https://$server_name$request_uri;
    }
}

# HTTPS Server
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name panel.yourdomain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/panel.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/panel.yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # Security Headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Client settings
    client_max_body_size 100M;
    client_body_timeout 300s;

    # Logging
    access_log /var/log/nginx/unified-panel-access.log;
    error_log /var/log/nginx/unified-panel-error.log;

    # Proxy to Rust backend
    location / {
        proxy_pass http://unified_panel_backend;
        proxy_http_version 1.1;

        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;

        proxy_buffering off;
        proxy_redirect off;
        proxy_cache_bypass $http_upgrade;

        # Timeouts
        proxy_connect_timeout 300s;
        proxy_send_timeout 300s;
        proxy_read_timeout 300s;
    }

    # Serve static files directly
    location /static/ {
        alias /opt/unified-panel/backend/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
        access_log off;
    }

    # Deny access to hidden files
    location ~ /\. {
        deny all;
        access_log off;
        log_not_found off;
    }
}
```

Enable and test:

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/unified-panel /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Reload Nginx
sudo systemctl reload nginx
```

### 8. Setup SSL with Let's Encrypt

```bash
# Install Certbot
sudo apt install -y certbot python3-certbot-nginx

# Obtain certificate (follow prompts)
sudo certbot --nginx -d panel.yourdomain.com

# Auto-renewal is configured automatically
# Test renewal:
sudo certbot renew --dry-run
```

### 9. Configure Firewall

```bash
# Enable UFW
sudo ufw enable

# Allow SSH
sudo ufw allow 22/tcp

# Allow HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Check status
sudo ufw status
```

## Service Management

### Control Service

```bash
# Start
sudo systemctl start unified-panel

# Stop
sudo systemctl stop unified-panel

# Restart
sudo systemctl restart unified-panel

# Reload (if supported)
sudo systemctl reload unified-panel

# Status
sudo systemctl status unified-panel

# Enable on boot
sudo systemctl enable unified-panel

# Disable on boot
sudo systemctl disable unified-panel
```

### View Logs

```bash
# Follow logs in real-time
sudo journalctl -u unified-panel -f

# View last 100 lines
sudo journalctl -u unified-panel -n 100

# View logs since today
sudo journalctl -u unified-panel --since today

# View logs with priority
sudo journalctl -u unified-panel -p err
```

## Monitoring

### Service Health

```bash
# Check if service is running
systemctl is-active unified-panel

# Check if service is enabled
systemctl is-enabled unified-panel

# Show service failure state
systemctl is-failed unified-panel
```

### Performance Monitoring

```bash
# View resource usage
systemctl status unified-panel

# Detailed resource usage
systemd-cgtop

# Service resource limits
systemctl show unified-panel
```

## Updates and Maintenance

### Update Application

```bash
# Stop service
sudo systemctl stop unified-panel

# Switch to panel user
sudo -u unified-panel -i

# Pull latest code
cd /opt/unified-panel
git pull

# Run new migrations
cd backend
sqlx migrate run

# Rebuild
cargo build --release

# Exit
exit

# Start service
sudo systemctl start unified-panel

# Verify
sudo systemctl status unified-panel
```

### Database Backups

Setup automated backups with systemd timers:

```bash
# Create backup script
sudo nano /usr/local/bin/backup-unified-panel.sh
```

```bash
#!/bin/bash
BACKUP_DIR="/var/backups/unified-panel"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p "$BACKUP_DIR"

# Backup database
pg_dump -U panel_user unified_panel | gzip > "$BACKUP_DIR/db_${DATE}.sql.gz"

# Keep only last 7 days
find "$BACKUP_DIR" -name "db_*.sql.gz" -mtime +7 -delete

# Optional: sync to remote storage
# rsync -az "$BACKUP_DIR/" user@backup-server:/backups/unified-panel/
```

```bash
# Make executable
sudo chmod +x /usr/local/bin/backup-unified-panel.sh

# Create systemd service
sudo nano /etc/systemd/system/unified-panel-backup.service
```

```ini
[Unit]
Description=Backup Unified Panel Database
After=postgresql.service

[Service]
Type=oneshot
User=postgres
ExecStart=/usr/local/bin/backup-unified-panel.sh
```

```bash
# Create timer
sudo nano /etc/systemd/system/unified-panel-backup.timer
```

```ini
[Unit]
Description=Daily Unified Panel Backup
Requires=unified-panel-backup.service

[Timer]
OnCalendar=02:00
Persistent=true

[Install]
WantedBy=timers.target
```

```bash
# Enable timer
sudo systemctl enable unified-panel-backup.timer
sudo systemctl start unified-panel-backup.timer

# Check timer status
sudo systemctl list-timers
```

## Troubleshooting

### Service Won't Start

```bash
# Check logs
sudo journalctl -u unified-panel -n 50 --no-pager

# Check configuration
/opt/unified-panel/backend/target/release/unified-panel --help

# Test database connection
sudo -u unified-panel psql "postgresql://panel_user:PASSWORD@localhost/unified_panel" -c "SELECT 1;"
```

### Permission Issues

```bash
# Fix ownership
sudo chown -R unified-panel:unified-panel /opt/unified-panel

# Check file permissions
ls -la /opt/unified-panel/backend/target/release/unified-panel
```

### Database Connection Errors

```bash
# Check PostgreSQL is running
sudo systemctl status postgresql

# Verify connection
sudo -u postgres psql -l

# Check pg_hba.conf
sudo nano /etc/postgresql/15/main/pg_hba.conf
```

### High Resource Usage

```bash
# Check resource limits
systemctl show unified-panel | grep -E 'CPU|Memory'

# Adjust limits in service file
sudo nano /etc/systemd/system/unified-panel.service

# Add under [Service]:
# MemoryLimit=2G
# CPUQuota=200%
```

## Security Hardening

### Service Isolation

Edit `/etc/systemd/system/unified-panel.service` and add:

```ini
[Service]
# Additional security options
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true
LockPersonality=true
PrivateDevices=true
```

### Fail2ban Integration

Create `/etc/fail2ban/filter.d/unified-panel.conf`:

```ini
[Definition]
failregex = ^.*Login attempt failed.*from <HOST>.*$
ignoreregex =
```

Create `/etc/fail2ban/jail.d/unified-panel.conf`:

```ini
[unified-panel]
enabled = true
port = http,https
filter = unified-panel
logpath = /var/log/nginx/unified-panel-access.log
maxretry = 5
bantime = 3600
findtime = 600
```

```bash
# Restart fail2ban
sudo systemctl restart fail2ban
```

## Performance Tuning

### PostgreSQL Optimization

```bash
# Edit PostgreSQL config
sudo nano /etc/postgresql/15/main/postgresql.conf
```

```ini
# Memory settings (adjust based on your server)
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
work_mem = 16MB

# Connections
max_connections = 100

# Checkpoints
checkpoint_completion_target = 0.9
wal_buffers = 16MB
```

```bash
# Restart PostgreSQL
sudo systemctl restart postgresql
```

### Nginx Optimization

```nginx
# Add to http block in /etc/nginx/nginx.conf
worker_processes auto;
worker_connections 1024;
keepalive_timeout 65;
gzip on;
gzip_types text/plain text/css application/json application/javascript;
```

---

This systemd deployment provides a production-ready, scalable, and maintainable hosting panel installation with native Linux integration.
