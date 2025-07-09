import axios from "axios";

const API_BASE_URL = process.env.REACT_APP_API_URL || "http://localhost:8000";

// Create axios instance
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request interceptor to add auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token");
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor to handle errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem("token");
      localStorage.removeItem("user");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authAPI = {
  login: (credentials) => api.post("/auth/login", credentials),
  register: (userData) => api.post("/auth/register", userData),
  googleLogin: (data) => api.post("/auth/google", data),
  logout: () => api.post("/auth/logout"),
  refreshToken: () => api.post("/auth/refresh"),
  getProfile: () => api.get("/auth/profile"),
  updateProfile: (data) => api.put("/auth/profile", data),
  changePassword: (data) => api.put("/auth/change-password", data),
};

// Dashboard API
export const dashboardAPI = {
  getStats: () => api.get("/dashboard/stats"),
  getRecentTrades: () => api.get("/dashboard/recent-trades"),
  getPerformanceChart: (period) =>
    api.get(`/dashboard/performance?period=${period}`),
  getActiveRobots: () => api.get("/dashboard/active-robots"),
};

// Trading Robots API
export const robotsAPI = {
  getAll: () => api.get("/robots"),
  getById: (id) => api.get(`/robots/${id}`),
  create: (data) => api.post("/robots", data),
  update: (id, data) => api.put(`/robots/${id}`, data),
  delete: (id) => api.delete(`/robots/${id}`),
  start: (id) => api.post(`/robots/${id}/start`),
  stop: (id) => api.post(`/robots/${id}/stop`),
  getPerformance: (id) => api.get(`/robots/${id}/performance`),
};

// Trades API
export const tradesAPI = {
  getAll: (params) => api.get("/trades", { params }),
  getById: (id) => api.get(`/trades/${id}`),
  create: (data) => api.post("/trades", data),
  close: (id, data) => api.post(`/trades/${id}/close`, data),
  getStats: () => api.get("/trades/stats"),
  export: (format, params) => api.get(`/trades/export/${format}`, { params }),
};

// Broker Connections API
export const brokersAPI = {
  getAll: () => api.get("/brokers"),
  getById: (id) => api.get(`/brokers/${id}`),
  create: (data) => api.post("/brokers", data),
  update: (id, data) => api.put(`/brokers/${id}`, data),
  delete: (id) => api.delete(`/brokers/${id}`),
  test: (id) => api.post(`/brokers/${id}/test`),
  activate: (id) => api.post(`/brokers/${id}/activate`),
  deactivate: (id) => api.post(`/brokers/${id}/deactivate`),
};

// Subscriptions API
export const subscriptionsAPI = {
  getCurrent: () => api.get("/subscriptions/current"),
  getPlans: () => api.get("/subscriptions/plans"),
  subscribe: (planId, paymentMethod) =>
    api.post("/subscriptions/subscribe", {
      plan_id: planId,
      payment_method: paymentMethod,
    }),
  cancel: () => api.post("/subscriptions/cancel"),
  updatePaymentMethod: (paymentMethod) =>
    api.put("/subscriptions/payment-method", {
      payment_method: paymentMethod,
    }),
  getInvoices: () => api.get("/subscriptions/invoices"),
};

// Users API (Admin)
export const usersAPI = {
  getAll: (params) => api.get("/admin/users", { params }),
  getById: (id) => api.get(`/admin/users/${id}`),
  update: (id, data) => api.put(`/admin/users/${id}`, data),
  delete: (id) => api.delete(`/admin/users/${id}`),
  activate: (id) => api.post(`/admin/users/${id}/activate`),
  deactivate: (id) => api.post(`/admin/users/${id}/deactivate`),
};

// Admin API
export const adminAPI = {
  getStats: () => api.get("/admin/stats"),
  getSystemHealth: () => api.get("/admin/system-health"),
  getAuditLogs: (params) => api.get("/admin/audit-logs", { params }),
  updateSystemSettings: (data) => api.put("/admin/settings", data),
  getSystemSettings: () => api.get("/admin/settings"),
};

// WebSocket connection
export const createWebSocketConnection = (token) => {
  const wsUrl = process.env.REACT_APP_WS_URL || "ws://localhost:8000/ws";
  const ws = new WebSocket(`${wsUrl}?token=${token}`);

  return ws;
};

// File upload utility
export const uploadFile = (file, endpoint) => {
  const formData = new FormData();
  formData.append("file", file);

  return api.post(endpoint, formData, {
    headers: {
      "Content-Type": "multipart/form-data",
    },
  });
};

// Error handling utility
export const handleApiError = (error) => {
  if (error.response) {
    // Server responded with error status
    const { status, data } = error.response;

    switch (status) {
      case 400:
        return data.message || "Bad request";
      case 401:
        return "Unauthorized access";
      case 403:
        return "Access forbidden";
      case 404:
        return "Resource not found";
      case 422:
        return data.detail || "Validation error";
      case 500:
        return "Internal server error";
      default:
        return data.message || "An error occurred";
    }
  } else if (error.request) {
    // Network error
    return "Network error. Please check your connection.";
  } else {
    // Other error
    return error.message || "An unexpected error occurred";
  }
};

export default api;
