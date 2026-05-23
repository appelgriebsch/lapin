/// Request access to a virtual host (AMQP 0-8 compatibility).
///
/// This method is a no-op in RabbitMQ and is retained only for compatibility
/// with AMQP 0-8 brokers. In AMQP 0-9-1 access control is handled at
/// connection time. The returned ticket value is ignored by modern brokers.
