SELECT
    jsonb_build_object(
        'first_name', users.first_name,
        'last_name', users.last_name,
        'email', users.email
    )
FROM users