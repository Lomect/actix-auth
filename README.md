# actix-auth
actix auth macro 

## dependencies
```
actix-web
actix-identity
```
## example
actix auth must have a params: `_id: Identity`
actix auth have a string by `let Some(ident) = identity.identity();`.

```
use actix_auth::login_required;

#[login_required]
pub async fn index(_id: Identity) -> HttpResponse {
    HttpResponse::Ok().json(ident)
}
```