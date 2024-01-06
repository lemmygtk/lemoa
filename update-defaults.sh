#!/bin/bash

BASE_URL="https://lemmy.ml/api/v3"

curl "$BASE_URL/user?username=kzhe@lemmy.zip" > src/examples/person.json
curl "$BASE_URL/community?name=asklemmy" > src/examples/community.json
curl "$BASE_URL/post?id=10133939" > src/examples/post.json
curl "$BASE_URL/site" > src/examples/site.json
