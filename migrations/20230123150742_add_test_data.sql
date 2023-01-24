-- TODO: テストデータを作る。型チェックが OK なことはわかったけどランタイムの挙動も一応見ておきたい。
BEGIN;
    -- memo
    INSERT INTO memo VALUES (1, 'title1', 'body1');
    INSERT INTO memo VALUES (2, 'title2', 'body2');
    INSERT INTO memo VALUES (3, 'title3', 'body3');
    -- secret.memo
    INSERT INTO secret.memo VALUES (1, 'secret1');
    INSERT INTO secret.memo VALUES (2, NULL);
    INSERT INTO secret.memo VALUES (3, 'secret3');
COMMIT;
