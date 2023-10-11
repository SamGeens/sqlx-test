CREATE TABLE
    person (
        id int,
        name varchar(255),
        age int,
        isActive boolean,
        PRIMARY KEY (id)
);

CREATE TABLE
    language (
         PERSON_id int,
         id int,
         name varchar(255),
         PRIMARY KEY (id),
         FOREIGN KEY (PERSON_id) REFERENCES person (id)
);

INSERT INTO person (id, name, age, isActive)
VALUES (0, 'test', 30, true);

INSERT INTO language (PERSON_id, id, name)
VALUES
(0, 0, 'English'),
(0, 1, 'Español'),
(0, 2, 'Chinese');
