CREATE TABLE teams (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    ping VARCHAR NOT NULL,
    label VARCHAR NOT NULL
);

CREATE TABLE memberships (
    id SERIAL PRIMARY KEY,
    fk_member INTEGER REFERENCES githubuser (id),
    fk_team INTEGER REFERENCES teams (id)
);

INSERT INTO teams (name, ping, label) VALUES
    ('Core', 'matrix-test-org/spec-core-team', 'proposal');

INSERT INTO memberships (fk_member, fk_team)
SELECT u.id, t.id
FROM githubuser u, teams t
WHERE t.ping = 'matrix-test-org/spec-core-team' AND (
    u.login = 'anoadragon453' OR
    u.login = 'ara4n' OR
    u.login = 'dbkr' OR
    u.login = 'erikjohnston' OR
    u.login = 'KitsuneRal' OR
    u.login = 'mujx' OR
    u.login = 'richvdh'OR 
    u.login = 'turt2live' OR
    u.login = 'uhoreg'
);
