CREATE TABLE occurances (
        thread_id VARCHAR NOT NULL,
        comment_id VARCHAR NOT NULL,
        acronym_id INTEGER NOT NULL REFERENCES acronyms (id),
        PRIMARY KEY (thread_id, acronym_id, comment_id)
);
