## setup

```bash
go mod init web
go mod tidy
```

## session
登录页面

```js
fetch('/login', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/x-www-form-urlencoded',
  },
  body: `username=user&password=password`,
})
  .then((response) => {
    // handle response
  })
  .catch((error) => {
    // handle error
  })
```
