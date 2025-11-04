# Unified Hosting Panel

A modern, high-performance hosting control panel built with **Rust**, **HTMX**, and **Tailwind CSS**. Designed for speed, security, and ease of use.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

### Core Functionality
- **Server Management** - Full CRUD operations for physical and virtual servers
- **User Management** - Role-based access control (Admin, Reseller, User)
- **Real-time Monitoring** - CPU, memory, disk, and network metrics
- **Authentication** - JWT-based auth with 2FA/TOTP support
- **Dark/Light Theme** - Smooth theme switching with persistent preferences

### Security
- Argon2id password hashing
- JWT token authentication
- Rate limiting on login attempts
- Audit logging for all critical actions
- SQL injection prevention (compile-time checked queries)
- XSS protection (auto-escaping templates)

### Automation
- **Bash Scripts** - Server health checks, database backups
- **Python Scripts** - Metrics collection and monitoring
- **Ansible Playbooks** - Server provisioning and configuration
- **n8n Workflows** - Complex automation workflows

### UI/UX
- Professional, polished design
- Smooth animations and transitions
- Glass morphism effects
- Responsive layout
- Loading states and skeleton screens
- HTMX for seamless interactivity (no page reloads)

## Tech Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust + Axum |
| Frontend | HTMX + Tailwind CSS + Alpine.js |
| Database | PostgreSQL + SQLx |
| Templating | Askama |
| Automation | Bash, Python, Ansible, n8n |
| Icons | Lucide Icons |

## Quick Start

### Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 15+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- Node.js 20+ (for Tailwind CSS compilation)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/xerudro/unified-panel.git
   cd unified-panel
   ```

2. **Set up the database**
   ```bash
   createdb unified_panel
   ```

3. **Configure environment**
   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Run migrations**
   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```

5. **Start the server**
   ```bash
   cargo run
   ```

6. **Access the panel**
   ```
   http://localhost:3000
   ```

### Default Credentials

```
Email: admin@unified-panel.local
Password: admin123
```

**⚠️ IMPORTANT: Change the default password immediately after first login!**

## Project Structure

```
unified-panel/
├── backend/                  # Rust backend
│   ├── src/
│   │   ├── api/             # API routes
│   │   ├── models/          # Data models
│   │   ├── services/        # Business logic
│   │   ├── handlers/        # Request handlers
│   │   ├── middleware/      # Auth, CORS, logging
│   │   └── utils/           # Utilities
│   ├── templates/           # HTML templates
│   ├── static/              # Static assets
│   └── migrations/          # Database migrations
│
├── automation/              # Automation scripts
│   ├── scripts/
│   │   ├── bash/           # Bash scripts
│   │   └── python/         # Python scripts
│   ├── ansible/            # Ansible playbooks
│   └── n8n/                # n8n workflows
│
└── docs/                   # Documentation
```

## Development

### Running in Development Mode

```bash
# Terminal 1: Run the backend with hot reload
cargo watch -x run

# Terminal 2: Watch Tailwind CSS changes (if needed)
npm run watch:css
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Linting
cargo clippy

# Formatting
cargo fmt
```

## API Documentation

### Authentication

All API endpoints (except `/api/auth/*`) require authentication via JWT token.

```bash
# Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password"}'

# Use token in subsequent requests
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  http://localhost:3000/api/servers
```

### Key Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/login` | User login |
| POST | `/api/auth/register` | User registration |
| GET | `/api/servers` | List servers |
| POST | `/api/servers` | Create server |
| GET | `/api/servers/:id` | Get server details |
| PUT | `/api/servers/:id` | Update server |
| DELETE | `/api/servers/:id` | Delete server |
| GET | `/api/servers/:id/metrics` | Get server metrics |
| GET | `/api/users` | List users |
| POST | `/api/users` | Create user |

Full API documentation: [docs/API.md](docs/API.md)

## Deployment

### Docker (Recommended)

```bash
docker-compose up -d
```

### Manual Deployment

1. Build the release binary:
   ```bash
   cargo build --release
   ```

2. Set up systemd service:
   ```bash
   sudo cp unified-panel.service /etc/systemd/system/
   sudo systemctl enable unified-panel
   sudo systemctl start unified-panel
   ```

3. Configure Nginx reverse proxy (see [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md))

## Automation

### Bash Scripts

```bash
# Health check
./automation/scripts/bash/server-health-check.sh

# Database backup
./automation/scripts/bash/backup-database.sh unified_panel
```

### Python Monitoring

```bash
cd automation/scripts/python
pip install -r requirements.txt
python monitor_metrics.py
```

### Ansible Provisioning

```bash
cd automation/ansible
ansible-playbook playbooks/setup-server.yml -i inventory/hosts.yml
```

### n8n Workflows

Import workflows from `automation/n8n/workflows/` into your n8n instance.

## Screenshots

### Landing Page
Modern, gradient-rich landing page with smooth animations.

### Dashboard
Clean dashboard with real-time metrics and activity feed.

### Dark Mode
Fully functional dark mode with smooth transitions.

## Roadmap

- [ ] VPS management (Hetzner, DigitalOcean, AWS)
- [ ] DNS management
- [ ] Email server management
- [ ] Website deployment automation
- [ ] Billing & invoicing
- [ ] Backup management
- [ ] File manager
- [ ] SSH terminal (web-based)
- [ ] Mobile app (Flutter)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/xerudro/unified-panel/issues)
- **Discussions**: [GitHub Discussions](https://github.com/xerudro/unified-panel/discussions)

## Acknowledgments

- Inspired by [VIP Super Hosting](https://github.com/xerudro/vip-super-hosting/)
- Built with amazing open-source tools and libraries
- Icons by [Lucide](https://lucide.dev/)

---

**Made with ❤️ using Rust, HTMX, and Tailwind CSS**
