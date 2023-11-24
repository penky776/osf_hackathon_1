api for managing an online forum (WIP)

## TODO
- [x] handle DELETE requests 
- [ ] secure user authentication 
- [ ] Error handling
- [ ] make documentation and more descriptive readme for the api




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

#### Get CSRF Token
`GET /get-csrf-token`

```bash
curl --location 'http://localhost:3000/get-csrf-token' \
--header 'Cookie: session_token=SESSION_TOKEN; username=USERNAME'
```

### 2. Posts

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

### 3. Comments

#### Add Comment

#### Delete Comment