import React from "react";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import { AuthProvider, useAuth } from "./contexts/AuthContext";
import Layout from "./components/layout/Layout";
import Login from "./components/auth/Login";
import Register from "./components/auth/Register";
import Dashboard from "./components/dashboard/Dashboard";
import LoadingSpinner from "./components/common/LoadingSpinner";

// Protected Route Component
const ProtectedRoute = ({ children }) => {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="large" />
      </div>
    );
  }

  return isAuthenticated ? children : <Navigate to="/login" replace />;
};

// Public Route Component (redirect if authenticated)
const PublicRoute = ({ children }) => {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="large" />
      </div>
    );
  }

  return !isAuthenticated ? children : <Navigate to="/dashboard" replace />;
};

// Main App Component
const AppContent = () => {
  return (
    <Router>
      <Routes>
        {/* Public Routes */}
        <Route
          path="/login"
          element={
            <PublicRoute>
              <Login />
            </PublicRoute>
          }
        />
        <Route
          path="/register"
          element={
            <PublicRoute>
              <Register />
            </PublicRoute>
          }
        />

        {/* Protected Routes */}
        <Route
          path="/dashboard"
          element={
            <ProtectedRoute>
              <Layout>
                <Dashboard />
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Robots Routes */}
        <Route
          path="/robots"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">Robôs de Trading</h1>
                  <p className="text-gray-600 mt-2">
                    Gerencie seus robôs de trading automatizado
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Trades Routes */}
        <Route
          path="/trades"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">Histórico de Trades</h1>
                  <p className="text-gray-600 mt-2">
                    Visualize e analise seus trades
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Brokers Routes */}
        <Route
          path="/brokers"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">
                    Conexões com Corretoras
                  </h1>
                  <p className="text-gray-600 mt-2">
                    Configure suas conexões com corretoras
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Subscription Routes */}
        <Route
          path="/subscription"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">Assinatura</h1>
                  <p className="text-gray-600 mt-2">
                    Gerencie sua assinatura e planos
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Settings Routes */}
        <Route
          path="/settings"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">Configurações</h1>
                  <p className="text-gray-600 mt-2">
                    Configure suas preferências
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Admin Routes */}
        <Route
          path="/admin"
          element={
            <ProtectedRoute>
              <Layout>
                <div className="p-8">
                  <h1 className="text-2xl font-bold">Painel Administrativo</h1>
                  <p className="text-gray-600 mt-2">
                    Gerencie usuários e sistema
                  </p>
                </div>
              </Layout>
            </ProtectedRoute>
          }
        />

        {/* Default redirect */}
        <Route path="/" element={<Navigate to="/dashboard" replace />} />

        {/* 404 Page */}
        <Route
          path="*"
          element={
            <div className="min-h-screen flex items-center justify-center">
              <div className="text-center">
                <h1 className="text-4xl font-bold text-gray-900 mb-4">404</h1>
                <p className="text-gray-600 mb-8">Página não encontrada</p>
                <button
                  onClick={() => window.history.back()}
                  className="btn-primary"
                >
                  Voltar
                </button>
              </div>
            </div>
          }
        />
      </Routes>
    </Router>
  );
};

// Root App Component with Providers
const App = () => {
  return (
    <AuthProvider>
      <AppContent />
    </AuthProvider>
  );
};

export default App;
