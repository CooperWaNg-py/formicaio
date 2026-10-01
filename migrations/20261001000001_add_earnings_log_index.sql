-- Position of the Transfer log within its block, so two identical payments in the same block
-- are kept apart and re-fetched logs are not stored twice. NULL for sync-progress marker rows
-- and for rows stored before this column existed (NULLs never conflict in a UNIQUE index).
ALTER TABLE earnings ADD COLUMN log_index INTEGER;
-- Sync now resumes right after each address's last sync marker (amount '0'), so legacy payment
-- rows above it (or all of them when there's no marker) will be fetched again with their log
-- index; drop them so they aren't counted twice.
DELETE FROM earnings
WHERE amount <> '0'
    AND log_index IS NULL
    AND block_number > COALESCE(
        (SELECT MAX(e2.block_number) FROM earnings e2
            WHERE e2.address = earnings.address AND e2.amount = '0'),
        -1
    );
CREATE UNIQUE INDEX IF NOT EXISTS idx_earnings_address_log
    ON earnings(address, block_number, log_index);
