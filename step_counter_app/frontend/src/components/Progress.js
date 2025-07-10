import React from "react";

const Progress = ({ dashboardData }) => {
  if (!dashboardData) {
    return <div className="loading">Carregando dados...</div>;
  }

  const { weight_goal } = dashboardData;
  const progressPercentage = weight_goal.progress_percentage;
  const remainingCalories = weight_goal.remaining_calories;
  const remainingKg = remainingCalories / 7000;

  return (
    <div className="card">
      <div className="progress-section">
        <div className="progress-header">
          <h2 className="progress-title">🎯 Meta: Perder 20kg</h2>
          <span className="progress-percentage">
            {progressPercentage.toFixed(1)}%
          </span>
        </div>

        <div className="progress-bar">
          <div
            className="progress-fill"
            style={{ width: `${Math.min(progressPercentage, 100)}%` }}
          ></div>
        </div>

        <div className="progress-info">
          <span>
            Queimadas:{" "}
            {weight_goal.calories_burned_so_far.toLocaleString("pt-BR")} cal
          </span>
          <span>
            Meta: {weight_goal.total_calories_needed.toLocaleString("pt-BR")}{" "}
            cal
          </span>
        </div>

        <div
          style={{
            textAlign: "center",
            marginTop: "15px",
            padding: "15px",
            background: "linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%)",
            borderRadius: "10px",
            color: "#333",
          }}
        >
          <div
            style={{
              fontSize: "1.2em",
              fontWeight: "600",
              marginBottom: "5px",
            }}
          >
            Faltam {remainingKg.toFixed(1)}kg para sua meta!
          </div>
          <div style={{ fontSize: "0.9em", opacity: "0.8" }}>
            {remainingCalories.toLocaleString("pt-BR")} calorias restantes
          </div>
        </div>
      </div>

      <div className="stats-grid">
        <div className="stat-item">
          <div className="stat-value">
            {dashboardData.total_steps.toLocaleString("pt-BR")}
          </div>
          <div className="stat-label">Passos Totais</div>
        </div>
        <div className="stat-item">
          <div className="stat-value">
            {dashboardData.total_distance_km.toFixed(1)}km
          </div>
          <div className="stat-label">Distância</div>
        </div>
      </div>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "1fr",
          gap: "10px",
          marginTop: "15px",
        }}
      >
        <div
          style={{
            textAlign: "center",
            padding: "15px",
            background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            borderRadius: "10px",
            color: "white",
          }}
        >
          <div className="stat-value">
            {dashboardData.total_calories_burned.toFixed(0)}
          </div>
          <div className="stat-label">Calorias Queimadas</div>
        </div>
      </div>

      {progressPercentage >= 100 && (
        <div
          style={{
            marginTop: "20px",
            padding: "20px",
            background: "linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)",
            borderRadius: "15px",
            textAlign: "center",
            color: "white",
          }}
        >
          <h3 style={{ margin: "0 0 10px 0", fontSize: "1.5em" }}>
            🎉 Parabéns!
          </h3>
          <p style={{ margin: "0", fontSize: "1.1em" }}>
            Você atingiu sua meta de perder 20kg!
          </p>
        </div>
      )}
    </div>
  );
};

export default Progress;
