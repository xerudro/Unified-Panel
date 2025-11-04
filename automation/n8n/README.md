# n8n Workflow Automation

## Overview

This directory contains n8n workflows for automating various hosting panel operations.

## Workflows

### 1. Server Provisioning (`server-provisioning.json`)

Automates the process of provisioning new servers:
- Receives webhook trigger from the panel
- Creates server on cloud provider (Hetzner, DigitalOcean, etc.)
- Waits for server to be ready
- Updates the panel with server details
- Sends success notification

### 2. Automated Backups (Future)

### 3. SSL Certificate Renewal (Future)

## Setup

1. Install n8n:
   ```bash
   npm install -g n8n
   ```

2. Start n8n:
   ```bash
   n8n start
   ```

3. Import workflows:
   - Navigate to http://localhost:5678
   - Click "Import workflow"
   - Select the JSON file from this directory

4. Configure credentials:
   - Add API keys for cloud providers
   - Configure webhook endpoints
   - Set authentication tokens

## Integration with Unified Panel

The panel can trigger n8n workflows via webhooks:

```bash
curl -X POST http://localhost:5678/webhook/unified-panel-provisioning \
  -H "Content-Type: application/json" \
  -d '{
    "server_name": "web-prod-03",
    "server_type": "cx11",
    "location": "nbg1"
  }'
```

## Environment Variables

Set these in your n8n environment:

```bash
N8N_BASIC_AUTH_ACTIVE=true
N8N_BASIC_AUTH_USER=admin
N8N_BASIC_AUTH_PASSWORD=your-secure-password
WEBHOOK_URL=http://localhost:5678
```
