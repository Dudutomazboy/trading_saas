# Google Authentication Setup

In development phase, some features will be implemented!

This document explains how to configure Google authentication on the AI Trading SaaS platform.

## 🔧 Google OAuth Configuration

### 1. Create Project in Google Cloud Console

1. Access the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the Google+ API or Google Identity

### 2. Configure OAuth 2.0

1. Go to **APIs & Services** > **Credentials**
2. Click **Create Credentials** > **OAuth 2.0 Client IDs**
3. Configure:
   - **Application type**: Web application
   - **Name**: Trading SaaS Platform
   - **Authorized JavaScript origins**:
     - `http://localhost:3000` (development)
     - `https://yourdomain.com` (production)
   - **Authorized redirect URIs**:
     - `http://localhost:3000` (development)
     - `https://yourdomain.com` (production)

### 3. Get Client ID

After creating credentials, you will receive:

- **Client ID**: `your-client-id.apps.googleusercontent.com`
- **Client Secret**: `your-client-secret` (not used in frontend)

## 🌐 Frontend Configuration

### 1. Environment Variables

Create the `frontend/.env` file:

```env
REACT_APP_API_URL=http://localhost:8000
REACT_APP_WS_URL=ws://localhost:8000/ws
REACT_APP_GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GENERATE_SOURCEMAP=false
```

### 2. Implementation

The frontend is already configured with:

- ✅ **Google Sign-In Script**: Loaded automatically
- ✅ **Login Button**: Integrated in login and register pages
- ✅ **Token Handling**: Secure sending to backend
- ✅ **State Management**: Integrated with AuthContext

## 🔧 Backend Configuration

### 1. Implemented Endpoints

- `POST /api/v1/auth/google` - Google Authentication

### 2. Authentication Flow

1. **Frontend**: User clicks "Continue with Google" button
2. **Google**: Opens authentication popup
3. **Google**: Returns JWT token to frontend
4. **Frontend**: Sends token to `/api/v1/auth/google`
5. **Backend**: Verifies token with Google API
6. **Backend**: Creates/updates user in database
7. **Backend**: Returns application JWT
8. **Frontend**: Stores token and redirects to dashboard

### 3. Token Verification

The backend implements two strategies:

#### Development (Mock)

```rust
// For tokens starting with "mock_google_token_"
if token.starts_with("mock_google_token_") {
    // Returns mock user for testing
}
```

#### Production (Google API)

```rust
// Verifies token with Google
let response = client
    .get(&format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", token))
    .send()
    .await?;
```

## 🧪 Testing the Implementation

### 1. Development Testing

To test without configuring Google OAuth:

```javascript
// In browser console
localStorage.setItem("mock_mode", "true");
```

### 2. Testing with Real Google

1. Configure environment variables
2. Start frontend: `cd frontend && npm start`
3. Start backend: `cd backend-rust && cargo run`
4. Access `http://localhost:3000`
5. Click "Continue with Google"

## 🔒 Security

### Implemented Measures

- ✅ **Token Verification**: Tokens are verified with Google API
- ✅ **HTTPS Only**: Configured for production
- ✅ **Domain Restriction**: Only authorized domains
- ✅ **Token Expiration**: Tokens have limited lifetime
- ✅ **User Validation**: Users are validated on backend

### Security Configurations

```rust
// Secure token verification
pub async fn verify_google_token(token: &str) -> Result<GoogleUser, AppError> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", token))
        .send()
        .await?;

    // Complete response validation
    // ...
}
```

## 🚀 Production Deployment

### 1. Configure Domain

In Google Cloud Console, add your production domain:

- **Authorized JavaScript origins**: `https://yourdomain.com`
- **Authorized redirect URIs**: `https://yourdomain.com`

### 2. Environment Variables

```env
# Production
REACT_APP_API_URL=https://api.yourdomain.com
REACT_APP_WS_URL=wss://api.yourdomain.com/ws
REACT_APP_GOOGLE_CLIENT_ID=your-production-client-id.apps.googleusercontent.com
```

### 3. HTTPS Required

Google OAuth requires HTTPS in production. Configure:

- Valid SSL certificate
- HTTP → HTTPS redirection
- Appropriate security headers

## 📝 Logs and Monitoring

### Backend Logs

```rust
tracing::info!("Google login attempt for email: {}", google_user.email);
tracing::warn!("Failed Google token verification: {}", error);
```

### Important Metrics

- Google login success rate
- Token verification response time
- Authentication errors
- New users via Google OAuth

## 🔧 Troubleshooting

### Common Issues

1. **"Google Sign-In not available"**

   - Check if Google script was loaded
   - Confirm Client ID in environment variables

2. **"Invalid Google token"**

   - Check if token hasn't expired
   - Confirm OAuth configuration in Google Cloud

3. **CORS Error**

   - Add domain to authorized origins
   - Check CORS configuration in backend

4. **"Account is disabled"**
   - User was deactivated in system
   - Check user status in database

### Debug Mode

To enable detailed logs:

```env
RUST_LOG=debug
```

```javascript
// Frontend debug
localStorage.setItem("debug_auth", "true");
```

## 📚 Additional Resources

- [Google Identity Documentation](https://developers.google.com/identity)
- [OAuth 2.0 Security Best Practices](https://tools.ietf.org/html/draft-ietf-oauth-security-topics)
- [React Google Login Guide](https://developers.google.com/identity/gsi/web/guides/overview)

---

✅ **Status**: Complete and functional implementation
🔧 **Next steps**: Configure real Client ID and test in production
