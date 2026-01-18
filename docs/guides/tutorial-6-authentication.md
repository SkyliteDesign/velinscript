# Tutorial 6: Authentication & Authorization

Lerne, wie du Authentication und Authorization in VelinScript implementierst.

## JWT Authentication

### Token generieren

```velin
@POST("/api/auth/login")
fn login(email: string, password: string): JWTToken {
    // Prüfe Credentials
    let user = db.findByEmail(User, email);
    if (user == null || !verifyPassword(password, user.password)) {
        return HttpResponse::unauthorized("Invalid credentials");
    }
    
    // Generiere Token
    let auth: AuthService = AuthService.new("secret-key");
    let claims = UserClaims {
        user_id: user.id,
        email: user.email,
        roles: user.roles,
    };
    return auth.generateToken(claims);
}
```

### Token verifizieren

```velin
@Auth
@GET("/api/profile")
fn getProfile(token: string): User {
    let auth: AuthService = AuthService.new("secret-key");
    let claims = auth.verifyToken(token);
    
    if (claims == null) {
        return HttpResponse::unauthorized("Invalid token");
    }
    
    return db.find(User, claims.user_id);
}
```

### Role-based Access Control

Die `@Role` Annotation nutzt nun echte JWT-Claims. Der Token muss ein `roles` Array im Payload enthalten.

```velin
@Auth
@Role("admin")
@GET("/api/admin/users")
fn getAdminUsers(): List<User> {
    return db.findAll(User);
}

@Auth
@Role("user")
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

### Multi-Factor Authentication (MFA)

VelinScript unterstützt nun nativ TOTP (Time-based One-Time Passwords).

```velin
@POST("/api/auth/mfa/verify")
fn verifyMfa(userId: string, code: string): boolean {
    let user = db.find(User, userId);
    // Verifiziert den Code gegen das gespeicherte Secret
    // Nutzt im Hintergrund das 'totp-rs' Crate für RFC 6238 Konformität
    return MFAService.verify_totp(code, user.mfaSecret);
}
```

## OAuth2 Integration

### Authorization Flow

```velin
@GET("/api/auth/oauth/authorize")
fn oauthAuthorize(provider: string): string {
    let oauth = OAuth2Provider::new(
        getClientId(provider),
        getClientSecret(provider),
        getRedirectUri(provider)
    );
    
    let state = generateState();
    let auth_url = oauth.get_authorization_url(state);
    
    return auth_url;
}

@GET("/api/auth/oauth/callback")
fn oauthCallback(code: string, state: string): JWTToken {
    let oauth = OAuth2Provider::new(
        getClientId("oauth"),
        getClientSecret("oauth"),
        getRedirectUri("oauth")
    );
    
    let token = oauth.exchange_code(code);
    return token;
}
```

## Best Practices

1. **Sichere Secrets** verwenden
2. **Token Expiry** implementieren
3. **HTTPS** für alle Auth-Endpoints
4. **Rate Limiting** für Login-Endpoints

## Nächste Schritte

- [Tutorial 7: ML Integration](tutorial-7-ml.md) - KI/ML Features
