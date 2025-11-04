#!/usr/bin/env python3
"""
Server Metrics Monitor
Collects system metrics and sends them to the Unified Panel API
"""

import psutil
import requests
import time
from datetime import datetime

API_URL = "http://localhost:3000/api/servers"
API_KEY = "your-api-key-here"  # Replace with actual API key
SERVER_ID = "your-server-id"  # Replace with actual server ID

def get_system_metrics():
    """Collect current system metrics"""
    return {
        "server_id": SERVER_ID,
        "cpu_usage": psutil.cpu_percent(interval=1),
        "memory_usage": psutil.virtual_memory().percent,
        "disk_usage": psutil.disk_usage('/').percent,
        "network_in": psutil.net_io_counters().bytes_recv,
        "network_out": psutil.net_io_counters().bytes_sent,
        "timestamp": datetime.utcnow().isoformat()
    }

def send_metrics(metrics):
    """Send metrics to the API"""
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {API_KEY}"
    }

    try:
        response = requests.post(
            f"{API_URL}/{SERVER_ID}/metrics",
            json=metrics,
            headers=headers,
            timeout=10
        )
        response.raise_for_status()
        print(f"✅ Metrics sent successfully at {metrics['timestamp']}")
        return True
    except requests.exceptions.RequestException as e:
        print(f"❌ Failed to send metrics: {e}")
        return False

def main():
    """Main monitoring loop"""
    print("🚀 Starting metrics monitor...")
    print(f"Server ID: {SERVER_ID}")
    print(f"API URL: {API_URL}")
    print("-" * 50)

    while True:
        try:
            metrics = get_system_metrics()
            print(f"\n📊 Current Metrics:")
            print(f"  CPU: {metrics['cpu_usage']:.1f}%")
            print(f"  Memory: {metrics['memory_usage']:.1f}%")
            print(f"  Disk: {metrics['disk_usage']:.1f}%")

            send_metrics(metrics)

            # Wait 60 seconds before next collection
            time.sleep(60)

        except KeyboardInterrupt:
            print("\n\n👋 Monitor stopped by user")
            break
        except Exception as e:
            print(f"❌ Error: {e}")
            time.sleep(60)

if __name__ == "__main__":
    main()
