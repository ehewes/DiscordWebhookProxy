-- Create an enum type for message status with only 'pending' and 'completed'
CREATE TYPE status_type AS ENUM ('pending', 'completed');

-- Create the webhooks table (unchanged)
CREATE TABLE webhooks (
    id SERIAL PRIMARY KEY,
    discord_webhook_id BIGINT UNIQUE NOT NULL,
    banned BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create the queue table with the updated status type
CREATE TABLE queue (
    id SERIAL PRIMARY KEY,
    webhook_id INTEGER NOT NULL REFERENCES webhooks(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    message JSONB NOT NULL,
    status status_type NOT NULL DEFAULT 'pending',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

CREATE INDEX idx_queue_webhook_status_created ON queue (webhook_id, status, created_at);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.webhook_id = OLD.webhook_id;
    NEW.created_at = OLD.created_at;
    NEW.message = OLD.message;
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to execute before updates
CREATE TRIGGER trig_queue_updated_at
BEFORE UPDATE ON queue
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- Create a function to delete completed messages
CREATE OR REPLACE FUNCTION delete_completed()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' THEN
        DELETE FROM queue WHERE id = NEW.id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to execute after updates to delete completed entries
CREATE TRIGGER trig_delete_completed
AFTER UPDATE ON queue
FOR EACH ROW
EXECUTE FUNCTION delete_completed();