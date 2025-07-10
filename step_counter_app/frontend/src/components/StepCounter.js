import React, { useState, useEffect, useRef } from "react";
import { stepAPI } from "../services/api";

const StepCounter = ({ onStepsAdded }) => {
  const [steps, setSteps] = useState(0);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");
  const [autoCounting, setAutoCounting] = useState(false);
  const [permissionGranted, setPermissionGranted] = useState(false);
  const lastStepTimeRef = useRef(0);
  const stepCountRef = useRef(0);

  // Thresholds for step detection
  const ACCELERATION_THRESHOLD = 12; // m/s^2, adjust as needed
  const STEP_DELAY = 400; // ms minimum delay between steps

  useEffect(() => {
    if (autoCounting && permissionGranted) {
      window.addEventListener("devicemotion", handleMotion);
    } else {
      window.removeEventListener("devicemotion", handleMotion);
    }

    return () => {
      window.removeEventListener("devicemotion", handleMotion);
    };
  }, [autoCounting, permissionGranted]);

  const handleMotion = (event) => {
    const acc = event.accelerationIncludingGravity;
    if (!acc) return;

    const accMagnitude = Math.sqrt(
      (acc.x || 0) * (acc.x || 0) +
        (acc.y || 0) * (acc.y || 0) +
        (acc.z || 0) * (acc.z || 0)
    );

    const now = Date.now();
    if (
      accMagnitude > ACCELERATION_THRESHOLD &&
      now - lastStepTimeRef.current > STEP_DELAY
    ) {
      lastStepTimeRef.current = now;
      stepCountRef.current += 1;
      setSteps(stepCountRef.current);
    }
  };

  const requestPermission = async () => {
    if (
      typeof DeviceMotionEvent !== "undefined" &&
      typeof DeviceMotionEvent.requestPermission === "function"
    ) {
      try {
        const response = await DeviceMotionEvent.requestPermission();
        if (response === "granted") {
          setPermissionGranted(true);
          setMessage("Permissão concedida para sensores de movimento.");
        } else {
          setMessage("Permissão negada para sensores de movimento.");
        }
      } catch (error) {
        setMessage("Erro ao solicitar permissão para sensores.");
      }
    } else {
      // Non iOS devices or browsers that don't require permission
      setPermissionGranted(true);
      setMessage("Sensores disponíveis.");
    }
  };

  const startAutoCounting = () => {
    if (!permissionGranted) {
      requestPermission();
    }
    setAutoCounting(true);
    setMessage("Contagem automática iniciada.");
  };

  const stopAutoCounting = () => {
    setAutoCounting(false);
    setMessage("Contagem automática pausada.");
  };

  const resetSteps = () => {
    stepCountRef.current = 0;
    setSteps(0);
    setMessage("Contador reiniciado.");
  };

  const sendStepsToBackend = async () => {
    if (steps <= 0) {
      setMessage("Nenhum passo para enviar.");
      return;
    }
    setLoading(true);
    setMessage("");
    try {
      const result = await stepAPI.addSteps(steps);
      setMessage(
        `Passos enviados: ${steps}. Distância: ${result.distance_km.toFixed(
          2
        )}km, Calorias: ${result.calories_burned.toFixed(1)}`
      );
      setSteps(0);
      stepCountRef.current = 0;
      if (onStepsAdded) {
        onStepsAdded();
      }
    } catch (error) {
      setMessage("Erro ao enviar passos. Tente novamente.");
      console.error("Error sending steps:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="card">
      <div className="header">
        <h1>🚶‍♂️ Contador de Passos Automático</h1>
        <p>
          Deixe o celular no bolso e caminhe. O app contará seus passos
          automaticamente.
        </p>
      </div>

      {message && (
        <div className={message.includes("Erro") ? "error" : "success"}>
          {message}
        </div>
      )}

      <div style={{ textAlign: "center", marginBottom: "20px" }}>
        <div style={{ fontSize: "3em", fontWeight: "700", color: "#333" }}>
          {steps.toLocaleString("pt-BR")}
        </div>
      </div>

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          marginBottom: "20px",
        }}
      >
        {!autoCounting ? (
          <button className="btn btn-primary" onClick={startAutoCounting}>
            Iniciar Contagem
          </button>
        ) : (
          <button className="btn btn-primary" onClick={stopAutoCounting}>
            Pausar Contagem
          </button>
        )}
        <button className="btn" onClick={resetSteps}>
          Reiniciar
        </button>
        <button
          className="btn btn-primary"
          onClick={sendStepsToBackend}
          disabled={loading || steps === 0}
        >
          {loading ? "Enviando..." : "Enviar Passos"}
        </button>
      </div>

      <div style={{ textAlign: "center", fontSize: "0.9em", color: "#666" }}>
        <p>
          Nota: A contagem automática depende do suporte do navegador e
          permissão para sensores de movimento.
        </p>
      </div>
    </div>
  );
};

export default StepCounter;
