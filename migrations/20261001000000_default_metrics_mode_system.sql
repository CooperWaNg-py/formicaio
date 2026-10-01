-- The shipped node binary doesn't serve an HTTP metrics endpoint, so HTTP mode (0), which
-- 20260514000000 set as the column default, leaves every node's stats unknown. Switch to
-- system stats (1).
UPDATE settings SET metrics_mode = 1 WHERE metrics_mode = 0;
