CREATE TABLE jobboard (
  jobboard_id BIGSERIAL PRIMARY KEY,
  jobboard_name VARCHAR(255) NOT NULL UNIQUE,
  url VARCHAR(255),
  account VARCHAR(255) NOT NULL,
  key VARCHAR(255),
  timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  verified BOOLEAN DEFAULT FALSE NOT NULL,
  active BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE company (
  company_id BIGSERIAL PRIMARY KEY,
  jobboard_id BIGINT REFERENCES jobboard(jobboard_id) NOT NULL,
  company_name VARCHAR(255) NOT NULL UNIQUE,
  logo VARCHAR(255),
  website VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  region VARCHAR(255),
  timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  verified BOOLEAN DEFAULT FALSE NOT NULL,
  active BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE vacancy (
  vacancy_id BIGSERIAL PRIMARY KEY,
  jobboard_id BIGINT REFERENCES jobboard(jobboard_id) NOT NULL,
  company_id BIGINT REFERENCES company(company_id) NOT NULL,
  job_title VARCHAR(255) NOT NULL,
  location VARCHAR(255),
  start_date TIMESTAMPTZ,
  directly BOOLEAN DEFAULT FALSE,
  hours INT4RANGE NOT NULL,
  positions SMALLINT DEFAULT 1,
  responsibilities VARCHAR(1200),
  skills VARCHAR(1200),
  conditions VARCHAR(1200),
  description VARCHAR(2000),
  url VARCHAR(255),
  commission SMALLINT CHECK (commission <= 10000),
  status VARCHAR(255) NOT NULL,
  verified BOOLEAN DEFAULT FALSE NOT NULL,
  active BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE application (
  application_id BIGSERIAL PRIMARY KEY,
  jobboard_id BIGINT REFERENCES jobboard(jobboard_id) NOT NULL,
  vacancy_id BIGSERIAL REFERENCES vacancy(vacancy_id) NOT NULL,
  first_name VARCHAR(255),
  last_name VARCHAR(255) NOT NULL,
  email VARCHAR(255),
  url_resume VARCHAR(255),
  url_extra_1 VARCHAR(255),
  url_extra_2 VARCHAR(255),
  url_extra_3 VARCHAR(255),
  verified BOOLEAN DEFAULT FALSE NOT NULL,
  status VARCHAR(255) NOT NULL
);
