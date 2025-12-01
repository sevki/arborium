# Filter and transform JSON data
.users
| map(select(.active == true))
| map({
    name: .name,
    email: .email,
    role: (.role // "user")
  })
| sort_by(.name)
| group_by(.role)
| map({
    role: .[0].role,
    count: length,
    members: [.[].name]
  })
