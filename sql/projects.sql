--- This may be some complex DB query that need highlight to understand simply e.g Postgres advance query.

SELECT
    projects.uuid,
    projects.organization_uuid,
    projects.name,
    projects.description,
    COUNT(prompts.uuid) AS prompts
FROM users
JOIN projects ON projects.user_uuid   = users.uuid
JOIN prompts  ON prompts.project_uuid = projects.uuid
WHERE
    users.uuid = ?