# Trading SaaS Frontend

In development phase, some features will be implemented!

Modern React.js frontend for the AI Trading SaaS platform with real-time dashboard and Google OAuth authentication.

## 🚀 Features

- **Modern UI**: Built with React 18 and Tailwind CSS
- **Real-time Dashboard**: Live trading data and performance charts
- **Authentication**: Google OAuth integration and JWT-based auth
- **Responsive Design**: Mobile-first approach with Tailwind CSS
- **Performance Charts**: Interactive charts with Chart.js
- **Real-time Updates**: WebSocket integration for live data
- **State Management**: Context API for global state
- **Type Safety**: PropTypes for component validation

## 🛠️ Technology Stack

- **Framework**: React 18 with functional components and hooks
- **Styling**: Tailwind CSS for utility-first styling
- **Charts**: Chart.js for performance visualization
- **Icons**: Heroicons for consistent iconography
- **HTTP Client**: Axios for API communication
- **Authentication**: Google Sign-In API + JWT
- **Real-time**: WebSocket for live updates
- **Build Tool**: Create React App with custom configuration

## 📁 Project Structure

```
src/
├── index.js                 # Application entry point
├── App.js                   # Main application component
├── index.css               # Global styles and Tailwind imports
├── components/             # Reusable UI components
│   ├── auth/              # Authentication components
│   │   ├── Login.js       # Login form with Google OAuth
│   │   └── Register.js    # Registration form
│   ├── dashboard/         # Dashboard components
│   │   ├── Dashboard.js   # Main dashboard layout
│   │   ├── StatsCards.js  # Statistics cards
│   │   ├── PerformanceChart.js # Trading performance chart
│   │   ├── RecentTrades.js # Recent trades table
│   │   └── ActiveRobots.js # Active robots status
│   ├── layout/            # Layout components
│   │   └── Layout.js      # Main layout with sidebar
│   └── common/            # Common UI components
│       └── LoadingSpinner.js # Loading indicator
├── contexts/              # React Context providers
│   └── AuthContext.js     # Authentication state management
└── services/              # API and external services
    └── api.js             # API client and endpoints
```

## 🚀 Quick Start

### Prerequisites

- Node.js 18 or later
- npm or yarn package manager

### Installation

1. **Navigate to frontend directory**

```bash
cd frontend
```

2. **Install dependencies**

```bash
npm install
```

3. **Set up environment variables**

```bash
cp .env.example .env
# Edit .env with your configuration
```

4. **Start development server**

```bash
npm start
```

The application will start on `http://localhost:3000`

## 🔧 Configuration

### Environment Variables

Create a `.env` file with the following variables:

```env
# API Configuration
REACT_APP_API_URL=http://localhost:8000
REACT_APP_WS_URL=ws://localhost:8000/ws

# Google OAuth
REACT_APP_GOOGLE_CLIENT_ID=your-google-client-id.apps.googleusercontent.com

# Build Configuration
GENERATE_SOURCEMAP=false
```

### Google OAuth Setup

1. **Create Google Cloud Project**

   - Go to [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project or select existing one
   - Enable Google+ API or Google Identity

2. **Configure OAuth 2.0**

   - Go to APIs & Services > Credentials
   - Create OAuth 2.0 Client ID
   - Add authorized origins: `http://localhost:3000`, `https://yourdomain.com`

3. **Update Environment**
   - Add your Client ID to `.env` file
   - The Google Sign-In script loads automatically

## 📊 Components Overview

### Authentication Components

#### Login.js

- Email/password login form
- Google OAuth integration
- Form validation and error handling
- Automatic redirect after successful login

#### Register.js

- User registration form
- Google OAuth registration
- Password confirmation validation
- Terms and conditions acceptance

### Dashboard Components

#### Dashboard.js

- Main dashboard layout
- Real-time data fetching
- WebSocket connection management
- Responsive grid layout

#### StatsCards.js

- Key performance indicators
- Profit/loss statistics
- Active robots count
- Total trades summary

#### PerformanceChart.js

- Interactive line chart
- Time period selection (1D, 7D, 30D, 1Y)
- Profit/loss visualization
- Responsive chart sizing

#### RecentTrades.js

- Recent trades table
- Trade status indicators
- Profit/loss highlighting
- Pagination support

#### ActiveRobots.js

- Active trading robots list
- Robot status indicators
- Start/stop controls
- Performance metrics

### Layout Components

#### Layout.js

- Main application layout
- Responsive sidebar navigation
- User profile dropdown
- Mobile menu toggle

### Common Components

#### LoadingSpinner.js

- Reusable loading indicator
- Customizable size and color
- Smooth animations

## 🎨 Styling

### Tailwind CSS Configuration

The project uses Tailwind CSS for styling with custom configuration:

```javascript
// tailwind.config.js
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: {
          50: "#eff6ff",
          500: "#3b82f6",
          600: "#2563eb",
          700: "#1d4ed8",
        },
      },
    },
  },
  plugins: [],
};
```

### Custom Styles

Global styles are defined in `src/index.css`:

- Tailwind base styles
- Custom component styles
- Chart.js overrides
- Responsive utilities

## 🔌 API Integration

### API Client Configuration

The API client is configured in `src/services/api.js`:

```javascript
// Automatic token attachment
api.interceptors.request.use((config) => {
  const token = localStorage.getItem("token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Automatic logout on 401
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem("token");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  }
);
```

### Available API Endpoints

- **Authentication**: Login, register, Google OAuth, profile
- **Dashboard**: Statistics, recent trades, performance data
- **Robots**: List, create, start/stop, performance
- **Trades**: History, statistics, export
- **Brokers**: Connections, testing, management
- **Subscriptions**: Plans, billing, invoices

## 🔄 State Management

### AuthContext

Global authentication state management:

```javascript
const {
  user,
  token,
  isAuthenticated,
  login,
  register,
  loginWithGoogle,
  logout,
} = useAuth();
```

### Local State

Components use React hooks for local state:

- `useState` for component state
- `useEffect` for side effects
- `useCallback` for memoized functions
- `useMemo` for computed values

## 🧪 Testing

### Run Tests

```bash
npm test
```

### Test Coverage

```bash
npm test -- --coverage
```

### Testing Strategy

- Unit tests for components
- Integration tests for API calls
- E2E tests for critical user flows

## 📱 Responsive Design

### Breakpoints

- **Mobile**: < 640px
- **Tablet**: 640px - 1024px
- **Desktop**: > 1024px

### Mobile Features

- Collapsible sidebar
- Touch-friendly buttons
- Optimized chart interactions
- Responsive tables

## 🚀 Performance Optimization

### Code Splitting

```javascript
// Lazy loading for routes
const Dashboard = lazy(() => import("./components/dashboard/Dashboard"));
```

### Memoization

```javascript
// Memoized components
const MemoizedChart = memo(PerformanceChart);
```

### Bundle Optimization

- Tree shaking for unused code
- Image optimization
- CSS purging with Tailwind
- Gzip compression

## 🔧 Development

### Available Scripts

```bash
# Start development server
npm start

# Build for production
npm run build

# Run tests
npm test

# Eject from Create React App (irreversible)
npm run eject

# Lint code
npm run lint

# Format code
npm run format
```

### Code Style

- ESLint for code linting
- Prettier for code formatting
- Consistent naming conventions
- Component documentation

### Adding New Components

1. **Create component file**

```javascript
// src/components/feature/NewComponent.js
import React from "react";

const NewComponent = ({ prop1, prop2 }) => {
  return <div className="p-4">{/* Component content */}</div>;
};

export default NewComponent;
```

2. **Add to parent component**

```javascript
import NewComponent from "./components/feature/NewComponent";
```

3. **Add tests**

```javascript
// src/components/feature/NewComponent.test.js
import { render, screen } from "@testing-library/react";
import NewComponent from "./NewComponent";

test("renders component", () => {
  render(<NewComponent />);
  // Test assertions
});
```

## 🐳 Docker

### Build Image

```bash
docker build -t trading-frontend .
```

### Run Container

```bash
docker run -p 3000:80 trading-frontend
```

### Multi-stage Build

The Dockerfile uses multi-stage build for optimization:

1. Build stage with Node.js
2. Production stage with Nginx

## 🚀 Deployment

### Production Build

```bash
npm run build
```

### Static Hosting

- Deploy `build/` folder to CDN
- Configure environment variables
- Set up custom domain
- Enable HTTPS

### Environment Configuration

- Production API URLs
- Google OAuth production credentials
- Analytics tracking codes
- Error monitoring setup

## 🔍 Monitoring

### Error Tracking

- Console error logging
- API error handling
- User action tracking
- Performance monitoring

### Analytics

- User behavior tracking
- Feature usage statistics
- Performance metrics
- Conversion tracking

## 🤝 Contributing

1. Follow React best practices
2. Use functional components with hooks
3. Write tests for new features
4. Follow Tailwind CSS conventions
5. Update documentation

## 📚 Resources

- [React Documentation](https://reactjs.org/docs/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Chart.js Documentation](https://www.chartjs.org/docs/)
- [Google Sign-In Documentation](https://developers.google.com/identity/gsi/web)

---

**Modern**: ⚛️ React 18 with latest features
**Responsive**: 📱 Mobile-first design
**Real-time**: ⚡ Live data updates
