INSERT INTO users ( mail, username, password_hash ) VALUES (
  'dummy@email.com', 'dummy', 'impossible'
);

INSERT INTO categories ( name, reimburstment_percentage, max_reimburstment ) VALUES (
  'Taxi', 80.00, 100.00
);

INSERT INTO claims ( user_id ) VALUES ( 1 );

INSERT INTO items ( claim_id, category_id, cost, reimburstment ) VALUES (
  1, 1, 100.00, 80.00
);



