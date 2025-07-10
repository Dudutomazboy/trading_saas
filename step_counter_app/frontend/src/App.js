import React, { useState, useEffect } from "react";
import StepCounter from "./components/StepCounter";
import Progress from "./components/Progress";
import { stepAPI } from "./services/api";
import "./index.css";

function App() {
  const [dashboardData, setDashboardData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const fetchDashboardData = async () => {
    try {
      setError("");
      const data = await stepAPI.getDashboard();
      setDashboardData(data);
    } catch (error) {
      console.error("Error fetching dashboard data:", error);
      setError("Erro ao carregar dados. Verifique se o servidor está rodando.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDashboardData();
  }, []);

  const handleStepsAdded = () => {
    // Refresh dashboard data when new steps are added
    fetchDashboardData();
  };

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleDateString("pt-BR", {
      day: "2-digit",
      month: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  if (loading) {
    return (
      <div className="container">
        <div className="loading">
          <h2>Carregando...</h2>
          <p>Conectando ao servidor...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container">
      {error && (
        <div className="error">
          {error}
          <button
            onClick={fetchDashboardData}
            style={{
              marginTop: "10px",
              padding: "8px 16px",
              background: "#c62828",
              color: "white",
              border: "none",
              borderRadius: "5px",
              cursor: "pointer",
            }}
          >
            Tentar Novamente
          </button>
        </div>
      )}

      <StepCounter onStepsAdded={handleStepsAdded} />

      <Progress dashboardData={dashboardData} />

      {dashboardData &&
        dashboardData.recent_records &&
        dashboardData.recent_records.length > 0 && (
          <div className="card">
            <div className="recent-records">
              <h3>📊 Registros Recentes</h3>
              {dashboardData.recent_records.map((record) => (
                <div key={record.id} className="record-item">
                  <div className="record-steps">
                    {record.steps.toLocaleString("pt-BR")} passos
                  </div>
                  <div className="record-details">
                    <div>{record.distance_km.toFixed(2)}km</div>
                    <div>{record.calories_burned.toFixed(0)} cal</div>
                    <div style={{ fontSize: "0.8em", opacity: "0.7" }}>
                      {formatDate(record.created_at)}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

      <div className="card" style={{ textAlign: "center", marginTop: "20px" }}>
        <h3 style={{ color: "#333", marginBottom: "15px" }}>
          ℹ️ Como Funciona
        </h3>
        <div style={{ fontSize: "0.9em", color: "#666", lineHeight: "1.6" }}>
          <p>
            <strong>Cálculo de Distância:</strong> Cada passo = 0,762m
          </p>
          <p>
            <strong>Cálculo de Calorias:</strong> 700 cal/hora caminhando a
            5km/h
          </p>
          <p>
            <strong>Meta de Peso:</strong> 7.000 calorias = 1kg perdido
          </p>
          <p>
            <strong>Objetivo:</strong> Queimar 140.000 calorias para perder 20kg
          </p>
        </div>
      </div>
    </div>
  );
}

export default App;
