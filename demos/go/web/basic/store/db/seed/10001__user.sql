INSERT INTO
  user (
    `id`,
    `username`,
    `role`,
    `email`,
    `nickname`,
    `open_id`,
    `password_hash`
  )
VALUES
  (
    101,
    'demohero',
    'HOST',
    'demo@usememos.com',
    'Demo Hero',
    hex(randomblob(16)),
    -- raw password: secret
    '$2a$14$ajq8Q7fbtFRQvXpdCq7Jcuy.Rx1h/L4J60Otx.gyNLbAYctGMJ9tK'
  );