curl -X POST localhost:7878/read_request \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -d "name=John+Doe&email=john.doe%40example.com&message=Hello+this+is+a+test+message"
