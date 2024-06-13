CREATE TYPE PlayerLevel AS ENUM ('beginner',
                                 'beginner+',
                                 'intermediate-',
                                 'intermediate',
                                 'intermediate+',
                                 'advanced-',
                                 'advanced',
                                 'advanced+')
CREATE TABLE IF NOT EXISTS players (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    display_name VARCHAR NOT NULL,
    player_level PlayerLevel NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
