INSERT INTO users ( mail, username, password_hash ) VALUES (
  'dummy@email.com', 'dummy', 'impossible'
);

INSERT INTO categories ( name, reimbursement_percentage, max_reimbursement ) VALUES (
  'Taxi', 80.00, 100.00
);

INSERT INTO claims ( user_id, total_cost, reimbursement ) VALUES ( 1, 100.00, 80.00 );

INSERT INTO items ( claim_id, category_id, cost, reimbursement ) VALUES (
  1, 1, 100.00, 80.00
);



