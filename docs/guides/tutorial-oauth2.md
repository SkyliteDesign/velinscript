# Tutorial: OAuth2/OpenID Connect Integration

Lerne, wie du OAuth2 und OpenID Connect in VelinScript verwendest.

## OAuth2 Decorator

Der `@OAuth2` Decorator schützt Endpoints mit OAuth2:

```velin
@OAuth2
@GET("/api/profile")
fn getProfile(): User {
    return currentUser();
}
```

## OpenID Connect

Verwende `@OIDC` für OpenID Connect:

```velin
@OIDC
@GET("/api/userinfo")
fn getUserInfo(): UserInfo {
    return fetchUserInfo();
}
```

## Multi-Factor Authentication

Aktiviere MFA mit `@MFA`:

```velin
@MFA
@POST("/api/sensitive-action")
fn performSensitiveAction(): ActionResult {
    return executeAction();
}
```

## Config-Setup

In `velin.config.json`:

```json
{
  "auth": {
    "provider": "oauth2",
    "oauth2": {
      "enabled": true,
      "clientId": "${OAUTH2_CLIENT_ID}",
      "clientSecret": "${OAUTH2_CLIENT_SECRET}",
      "authUrl": "https://oauth.provider.com/authorize",
      "tokenUrl": "https://oauth.provider.com/token"
    },
    "oidc": {
      "enabled": true,
      "issuerUrl": "https://oidc.provider.com"
    },
    "mfa": true
  }
}
```

## Vollständiges Beispiel

Siehe [examples/oauth2-api.velin](../../examples/oauth2-api.velin) für ein vollständiges Beispiel.
