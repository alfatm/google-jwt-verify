# Google JSON Web Token Verify
This can be used to verify Google JWT tokens. Google's public keys are automatically fetched
and cached according to the returned Cache-Control headers. Most requests to verify a token
through this library will not wait for an HTTP request

For more info: https://developers.google.com/identity/sign-in/web/backend-auth

## Quick Start
```rust
 //If you don't have a client id, get one from here: https://console.developers.google.com/
 let client_id = "37772117408-qjqo9hca513pdcunumt7gk08ii6te8is.apps.googleusercontent.com";
 let token = "...";// Obtain a signed token from Google
 let client = Client::builder(&client_id).build();
 let id_token = client.verify_id_token(&token)?;
 
 //use the token to obtain information about the verified user
 let user_id = id_token.get_claims().get_subject();
 let email = id_token.get_payload().get_email();
 let name = id_token.get_payload().get_name();
```
