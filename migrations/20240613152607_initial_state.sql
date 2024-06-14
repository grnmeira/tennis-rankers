-- login/access management
CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- players
CREATE TYPE PlayerLevel AS ENUM ('beginner',
                                 'beginner+',
                                 'intermediate-',
                                 'intermediate',
                                 'intermediate+',
                                 'advanced-',
                                 'advanced',
                                 'advanced+');

CREATE TYPE PlayerStatus AS ENUM ('active',
                                  'injured',
                                  'temporarily_unavailable');

CREATE TYPE Gender AS ENUM ('M', 'F', 'O');

CREATE TABLE IF NOT EXISTS players (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    display_name VARCHAR NOT NULL,
    player_level PlayerLevel NOT NULL,
    status PlayerStatus NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- score
CREATE TYPE ScoreReason AS ENUM('other',
                                'victory',
                                'loss',
                                'bonus');

CREATE TABLE IF NOT EXISTS score_ledger (
    player_id uuid NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    reason ScoreReason DEFAULT 'other' NOT NULL,
    comment VARCHAR,
    score INTEGER DEFAULT 0 NOT NULL,
    PRIMARY KEY (player_id, created_at)
);

-- matches
CREATE TABLE IF NOT EXISTS matches (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    player1 uuid REFERENCES players(id) NOT NULL,
    player2 uuid REFERENCES players(id) NOT NULL,
    match_date DATE,
    CHECK (player1 != player2)
);

CREATE INDEX match_player1_index ON matches(player1);
CREATE INDEX match_player2_index ON matches(player2);

--venues
CREATE TABLE IF NOT EXISTS venues (
);

CREATE TABLE IF NOT EXISTS favorite_venus (
);