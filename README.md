# OAuth API

## About

This program handles all Xiler API routes that start with `/oauth/`.

## Documentation

**Routes**: [docs.xiler.net/api/oauth](https://docs.xiler.net/api.oauth)  
**Objects**: [docs.xiler.net/api/objects](https://docs.xiler.net/api.oauth)  

# PRE DOCS

## Endpoints

### `/code` *(private)*

JSON Request Body:
* `token` | The user their current token (which is granted on site login)
* `app` | The application ID that will be linked to the token
* `user` | The uuid of the user that will grant access

Returns:
```json
{
  "status": {
    "code": 201,
    "message": "Successfully generated an auth code"
  },
  "data": {
    "code": "hhrublkdd1ytqmjnkddeeuxnsilw8mavroyrxfxqu0adikrcpf0ivchithkkcyjj",
    "app": "83ed1"
  }
}
```

### `/token`

JSON Request Body:
* `code` | The code that has been received by code route
* `app` | The application ID that is linked to the token

Returns:
```json
{
  "status": {
    "code": 0,
    "message": "Successfully created a new token"
  },
  "data": {
    "access_token": "d19ca43034765d2c9ca51f3364ac0dcc.ba54cdfb7a125c2cd13be08#2c23fc63",
    "app": "83ed1",
    "refresh_token": "3eb016cba372e82233103cbc46b1e87f.2d78f398d7e6ba2e6598f8e#6be110b2",
    "token_type": 1,
    "expires_in": 599825
  }
}
```
