import pika
import time

time.sleep(10)  # wait for RabbitMQ to start

connection = pika.BlockingConnection(
    pika.ConnectionParameters('rabbitmq')
)
channel = connection.channel()
channel.queue_declare(queue='demo')

print("Publishing messages...")
for i in range(1, 11):
    message = f"Message {i}"
    channel.basic_publish(exchange='', routing_key='demo', body=message)
    print(f"Sent: {message}")
    time.sleep(1)

connection.close()
