# Deployment Guide

This guide covers deploying the Unified Hosting Panel to production.

## Prerequisites

- Linux server (Ubuntu 22.04 LTS recommended)
- PostgreSQL 15+
- Nginx (for reverse proxy)
- Domain name with DNS configured

## Deployment Steps

### 1. Server Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev postgresql nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Database Setup

```bash
# Create database and user
sudo -u postgres psql << EOF
CREATE DATABASE unified_panel;
CREATE USER panel_user WITH ENCRYPTED PASSWORD 'your-secure-password';
GRANT ALL PRIVILEGES ON DATABASE unified_panel TO panel_user;
EOF
```

### 3. Application Setup

```bash
# Create app directory
sudo mkdir -p /opt/unified-panel
sudo chown $USER:$USER /opt/unified-panel

# Clone and build
cd /opt/unified-panel
git clone https://github.com/xerudro/unified-panel.git .
cd backend

# Configure environment
cp .env.example .env
nano .env  # Edit with production values

# Run migrations
cargo install sqlx-cli
sqlx migrate run

# Build release
cargo build --release
```

### 4. Systemd Service

Create `/etc/systemd/system/unified-panel.service`:

```ini
[Unit]
Description=Unified Hosting Panel
After=network.target postgresql.service

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/opt/unified-panel/backend
Environment="DATABASE_URL=postgresql://panel_user:password@localhost/unified_panel"
Environment="JWT_SECRET=your-super-secret-jwt-key"
Environment="RUST_LOG=info"
ExecStart=/opt/unified-panel/backend/target/release/unified-panel
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable unified-panel
sudo systemctl start unified-panel
sudo systemctl status unified-panel
```

### 5. Nginx Configuration

Create `/etc/nginx/sites-available/unified-panel`:

```nginx
upstream unified_panel {
    server 127.0.0.1:3000;
}

server {
    listen 80;
    server_name panel.yourdomain.com;

    location / {
        return 301 https://$server_name$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name panel.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/panel.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/panel.yourdomain.com/privkey.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    client_max_body_size 100M;

    location / {
        proxy_pass http://unified_panel;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    location /static/ {
        alias /opt/unified-panel/backend/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }
}
```

Enable and test:

```bash
sudo ln -s /etc/nginx/sites-available/unified-panel /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 6. SSL Certificate

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d panel.yourdomain.com
```

### 7. Firewall

```bash
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

## Docker Deployment

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: unified_panel
      POSTGRES_USER: panel_user
      POSTGRES_PASSWORD: your-secure-password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - panel_network

  panel:
    build: ./backend
    ports:
      - "3000:3000"
    environment:
      DATABASE_URL: postgresql://panel_user:your-secure-password@postgres:5432/unified_panel
      JWT_SECRET: your-super-secret-jwt-key
      RUST_LOG: info
    depends_on:
      - postgres
    networks:
      - panel_network
    restart: unless-stopped

volumes:
  postgres_data:

networks:
  panel_network:
```

Deploy:

```bash
docker-compose up -d
```

## Monitoring

### Logs

```bash
# Application logs
sudo journalctl -u unified-panel -f

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

### Health Checks

```bash
# Check service status
sudo systemctl status unified-panel

# Test endpoint
curl http://localhost:3000/
```

## Backup

### Database Backup

```bash
# Automated backup (cron)
0 2 * * * /opt/unified-panel/automation/scripts/bash/backup-database.sh unified_panel
```

### Application Backup

```bash
# Backup configuration
tar -czf panel-backup-$(date +%Y%m%d).tar.gz \
  /opt/unified-panel/backend/.env \
  /opt/unified-panel/backend/static
```

## Security Hardening

1. **Change default credentials** immediately
2. **Use strong JWT secret** (64+ random characters)
3. **Enable 2FA** for all admin accounts
4. **Regular updates**: `sudo apt update && sudo apt upgrade`
5. **Monitor logs** for suspicious activity
6. **Fail2ban** for brute force protection:
   ```bash
   sudo apt install fail2ban
   sudo systemctl enable fail2ban
   ```

## Troubleshooting

### Service won't start

```bash
# Check logs
sudo journalctl -u unified-panel -n 50

# Check permissions
sudo chown -R www-data:www-data /opt/unified-panel
```

### Database connection errors

```bash
# Test database connection
psql -U panel_user -d unified_panel -h localhost

# Check PostgreSQL status
sudo systemctl status postgresql
```

### Nginx errors

```bash
# Test configuration
sudo nginx -t

# Reload configuration
sudo systemctl reload nginx
```

## Scaling

For high-traffic deployments:

1. **Load Balancing**: Use multiple application instances behind a load balancer
2. **Database Replication**: Set up PostgreSQL streaming replication
3. **Caching**: Implement Redis for session storage and caching
4. **CDN**: Use a CDN for static assets

## Support

For deployment issues, please open an issue on GitHub or contact support.
