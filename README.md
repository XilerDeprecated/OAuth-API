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
    "code": "chzmnb8n808nwnre4echvh6bvt86kuk1hrlrchvh77lzu43xqsb1fg93zb3ww9lj"
  }
}
```

### `/token`

JSON Request Body:
* `code` | The code that has been received by code route

Returns:
```json
{
  "status": {
    "code": 0,
    "message": "Successfully created a new token"
  },
  "data": {
    "access_token": "4eabde45ff5626562cf8e9dc7c0abf8f.9c8d9442383b05476310e0e#66c66b94",
    "app": "83ed1",
    "refresh_token": "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae",
    "token_type": 1,
    "expires_in": 604800
  }
}
```
