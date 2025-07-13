import pika
import time

def callback(ch, method, properties, body):
    print(f"Received: {body.decode()}")

while True:
    try:
        connection = pika.BlockingConnection(
            pika.ConnectionParameters('rabbitmq')
        )
        break
    except pika.exceptions.AMQPConnectionError:
        print("Waiting for RabbitMQ...")
        time.sleep(2)

channel = connection.channel()
channel.queue_declare(queue='demo')

print("Waiting for messages...")
channel.basic_consume(queue='demo', on_message_callback=callback, auto_ack=True)

try:
    channel.start_consuming()
except KeyboardInterrupt:
    print("Consumer stopped.")
