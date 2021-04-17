CREATE TABLE symptoms (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE diseases (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    symptoms int[] NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE departments (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    diseases int[] NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE doctors (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    occupied BOOLEAN NOT NULL,
    department INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_department
        FOREIGN KEY (department)
            REFERENCES departments(id)
            ON DELETE SET NULL
);

CREATE TABLE chances (
    disease INT NOT NULL,
    chance INT NOT NULL,
    PRIMARY KEY (disease),
    CONSTRAINT fk_disease
        FOREIGN KEY (disease)
             REFERENCES diseases(id)
             ON DELETE SET NULL
)