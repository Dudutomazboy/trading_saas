import axios from "axios";

const API_BASE_URL = process.env.REACT_APP_API_URL || "http://localhost:8000";

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export const stepAPI = {
  // Add new step record
  addSteps: async (steps) => {
    const response = await api.post("/steps", { steps });
    return response.data;
  },

  // Get recent step records
  getSteps: async (limit = 10) => {
    const response = await api.get(`/steps?limit=${limit}`);
    return response.data;
  },

  // Get dashboard data
  getDashboard: async () => {
    const response = await api.get("/dashboard");
    return response.data;
  },

  // Delete step record
  deleteStepRecord: async (stepId) => {
    const response = await api.delete(`/steps/${stepId}`);
    return response.data;
  },

  // Health check
  healthCheck: async () => {
    const response = await api.get("/");
    return response.data;
  },
};

export default api;
