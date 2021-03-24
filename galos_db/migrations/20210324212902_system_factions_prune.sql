DROP INDEX system_factions_system_address_faction_id_idx;


ALTER TABLE factions
    ADD COLUMN government Government,
    ADD COLUMN allegiance Allegiance;

UPDATE factions f SET
    government = sf.government,
    allegiance = sf.allegiance
FROM system_factions sf
WHERE f.id = sf.faction_id;

ALTER TABLE factions
    ALTER COLUMN government SET NOT NULL,
    ALTER COLUMN allegiance SET NOT NULL;

ALTER TABLE system_factions
    DROP COLUMN government,
    DROP COLUMN allegiance;
