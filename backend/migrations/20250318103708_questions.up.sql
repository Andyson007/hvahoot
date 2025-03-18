CREATE TABLE hvahoots (
  id SERIAL PRIMARY KEY,
  owner INTEGER REFERENCES clients(id)
);

CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  hvahoot INTEGER REFERENCES hvahoots(id),
  question TEXT NOT NULL,
  answers TEXT[4],
  correct INTEGER
);

CREATE TABLE access (
  hvahoot INTEGER REFERENCES hvahoots(id),
  client INTEGER REFERENCES clients(id)
);


