CREATE TABLE queue (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  scheduled_for TIMESTAMP WITH TIME ZONE NOT NULL,
  failed_attempts INT NOT NULL,
  status INT NOT NULL,
  message JSONB NOT NULL
);
CREATE INDEX index_queue_on_scheduled_for ON queue (scheduled_for);
CREATE INDEX index_queue_on_status ON queue (status);
