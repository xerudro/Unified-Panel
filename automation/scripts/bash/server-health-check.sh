#!/bin/bash
# Server Health Check Script
# Usage: ./server-health-check.sh [server_ip]

set -euo pipefail

SERVER_IP="${1:-localhost}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "==================================="
echo "Server Health Check - $TIMESTAMP"
echo "Server: $SERVER_IP"
echo "==================================="

# CPU Usage
echo -e "\n📊 CPU Usage:"
top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1"%"}'

# Memory Usage
echo -e "\n💾 Memory Usage:"
free -h | awk '/^Mem:/ {printf "Used: %s / %s (%.2f%%)\n", $3, $2, ($3/$2)*100}'

# Disk Usage
echo -e "\n💿 Disk Usage:"
df -h / | awk 'NR==2 {printf "Used: %s / %s (%s)\n", $3, $2, $5}'

# Network Stats
echo -e "\n🌐 Network Status:"
if ping -c 1 8.8.8.8 &> /dev/null; then
    echo "✅ Internet connection: OK"
else
    echo "❌ Internet connection: FAILED"
fi

# System Load
echo -e "\n⚡ System Load:"
uptime | awk -F'load average:' '{print $2}'

# Check critical services
echo -e "\n🔧 Services Status:"
for service in nginx postgresql redis; do
    if systemctl is-active --quiet $service 2>/dev/null; then
        echo "✅ $service: Running"
    else
        echo "❌ $service: Not running"
    fi
done

echo -e "\n==================================="
echo "Health check completed!"
echo "==================================="
