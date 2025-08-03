#!/bin/bash

set -e

echo "🐳 Building and starting the message queue demo..."
docker compose up --build -d

echo "⌛ Waiting for containers to initialize..."
sleep 15

echo "📤 Messages are being published to the queue..."
echo "🛑 Simulating consumer failure in 5 seconds..."
sleep 5

dockaer compose stop consumer
echo "💥 Consumer stopped. Publisher continues to publish messages."

sleep 10

echo "🔁 Restarting the consumer to process missed messages..."
docker compose start consumer

echo "✅ Consumer is now running. You should see it receive all messages sent during downtime."
echo ""
echo "🌐 Optional: Open RabbitMQ UI at http://localhost:15672 (user: guest / pass: guest)"
echo ""
echo "🧹 When finished, run: docker compose down"
