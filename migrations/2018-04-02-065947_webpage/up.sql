CREATE TABLE projects (
  `name` VARCHAR NOT NULL,
  body Text NOT NULL,
  link VARCHAR NOT NULL,
  file_link VARCHAR NOT NULL,
)

CREATE TABLE posts (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR NOT NULL,
  body Text NOT NULL,
  link VARCHAR NOT NULL,
  Date VARCHAR NOT NULL,
)