WITH datemax AS ( SELECT MAX(date_) AS datemax FROM entries )
SELECT * FROM
entries JOIN datemax
WHERE date_ > (datemax - (1000 * 60 * ?1))
