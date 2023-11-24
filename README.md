api for managing an online forum

## TODO
- [x] handle DELETE requests 
- [ ] secure user authentication 
- [ ] Error handling
- [x] make documentation and more descriptive readme for the api


## Table of Contents

* [Dependencies](#dependencies)
* [Usage](#usage)
* [Documentation](#documentation)
    - [Authenticate](#1-authenticate)
        - [Login](#log-in)
        - [Register](#register)
    - [User](#2-user)
        - [Delete User](#delete-user)
        - [Get CSRF Token](#get-csrf-token)
    - [Posts](#2-posts)
        - [Add Post](#add-post)
        - [Delete Post](#delete-post)
    - [Comments](#3-comments)

## Dependencies

The only dependencies are rust and cargo.

## Usage

``` bash
git clone git@github.com:penky776/osf_hackathon_1.git
# OR
git clone https://github.com/penky776/osf_hackathon_1.git
```
To start the server, cd into the directory and:
``` bash
cargo run
```

## Documentation

### 1. Authenticate

#### Log in 
`POST /login`

```bash
curl --location 'http://localhost:3000/login' \
--data-urlencode 'username=USERNAME' \
--data-urlencode 'password=PASSWORD' -v
```

#### Register
`POST /register`

```bash
curl --location 'http://localhost:3000/register' \
--data-urlencode 'username=USERNAME' \
--data-urlencode 'password=PASSWORD' -v
```

### 2. User

#### Delete User
`DELETE /deleteuser`

```bash
curl --location --request DELETE 'http://localhost:3000/deleteuser' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME'
```

#### Get CSRF Token
`GET /get-csrf-token`

```bash
curl --location 'http://localhost:3000/get-csrf-token' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME'
```

### 3. Posts

#### Add Post 
`POST /addpost`

```bash
curl --location 'http://localhost:3000/addpost' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME' \
--data-urlencode 'csrf_token=CSRF_TOKEN' \
--data-urlencode 'title=POST_TITLE' \
--data-urlencode 'body=POST_BODY'
```

#### Delete Post 
`POST /deletepost`

```bash
curl --location 'http://localhost:3000/deletepost' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME' \
--data-urlencode 'id=POST_ID' \
--data-urlencode 'csrf_token=CSRF_TOKEN'
```

### 4. Comments

#### Add Comment
`POST /addcomment`

```bash
curl --location 'http://localhost:3000/addcomment' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME' \
--data-urlencode 'body=COMMENT_BODY' \
--data-urlencode 'post_id=POST_ID' \
--data-urlencode 'csrf_token=CSRF_TOKEN'
```

#### Delete Comment
`POST /deletecomment`

```bash
curl --location 'http://localhost:3000/deletecomment' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME' \
--data-urlencode 'id=COMMENT_ID' \
--data-urlencode 'csrf_token=CSRF_TOKEN'
```