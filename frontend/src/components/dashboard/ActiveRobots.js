import React from "react";
import {
  PlayIcon,
  PauseIcon,
  CogIcon,
  ChartBarIcon,
} from "@heroicons/react/24/outline";

const ActiveRobots = ({ robots }) => {
  const formatCurrency = (value) => {
    return new Intl.NumberFormat("pt-BR", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 2,
    }).format(value || 0);
  };

  const formatPercentage = (value) => {
    const num = parseFloat(value || 0);
    return `${num >= 0 ? "+" : ""}${num.toFixed(2)}%`;
  };

  const getStatusColor = (status) => {
    switch (status) {
      case "active":
        return "bg-green-500";
      case "paused":
        return "bg-yellow-500";
      case "stopped":
        return "bg-gray-400";
      case "error":
        return "bg-red-500";
      default:
        return "bg-gray-400";
    }
  };

  const getStatusLabel = (status) => {
    switch (status) {
      case "active":
        return "Ativo";
      case "paused":
        return "Pausado";
      case "stopped":
        return "Parado";
      case "error":
        return "Erro";
      default:
        return status;
    }
  };

  if (!robots || robots.length === 0) {
    return (
      <div className="card">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Robôs Ativos</h3>
        <div className="text-center py-8">
          <CogIcon className="h-12 w-12 text-gray-400 mx-auto mb-4" />
          <div className="text-gray-500">Nenhum robô ativo</div>
          <button className="btn-primary mt-4">Criar Primeiro Robô</button>
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-medium text-gray-900">Robôs Ativos</h3>
        <button className="btn-secondary text-sm">Ver Todos</button>
      </div>

      <div className="space-y-4">
        {robots.map((robot) => (
          <div
            key={robot.id}
            className="border border-gray-200 rounded-lg p-4 hover:bg-gray-50"
          >
            <div className="flex items-center justify-between mb-3">
              <div className="flex items-center">
                <div
                  className={`w-3 h-3 rounded-full ${getStatusColor(
                    robot.status
                  )} mr-3`}
                ></div>
                <div>
                  <h4 className="text-sm font-medium text-gray-900">
                    {robot.name}
                  </h4>
                  <p className="text-xs text-gray-500">{robot.strategy}</p>
                </div>
              </div>
              <div className="flex items-center space-x-2">
                {robot.status === "active" ? (
                  <button className="p-1 text-yellow-600 hover:text-yellow-700">
                    <PauseIcon className="h-4 w-4" />
                  </button>
                ) : (
                  <button className="p-1 text-green-600 hover:text-green-700">
                    <PlayIcon className="h-4 w-4" />
                  </button>
                )}
                <button className="p-1 text-gray-600 hover:text-gray-700">
                  <ChartBarIcon className="h-4 w-4" />
                </button>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <span className="text-gray-600">Status:</span>
                <span
                  className={`ml-2 font-medium ${
                    robot.status === "active"
                      ? "text-green-600"
                      : robot.status === "error"
                      ? "text-red-600"
                      : "text-yellow-600"
                  }`}
                >
                  {getStatusLabel(robot.status)}
                </span>
              </div>
              <div>
                <span className="text-gray-600">Trades:</span>
                <span className="ml-2 font-medium text-gray-900">
                  {robot.total_trades || 0}
                </span>
              </div>
            </div>

            <div className="mt-3 pt-3 border-t border-gray-200">
              <div className="flex items-center justify-between text-sm">
                <div>
                  <span className="text-gray-600">P&L:</span>
                  <span
                    className={`ml-2 font-medium ${
                      robot.total_pnl >= 0 ? "text-green-600" : "text-red-600"
                    }`}
                  >
                    {formatCurrency(robot.total_pnl)}
                  </span>
                </div>
                <div>
                  <span className="text-gray-600">Retorno:</span>
                  <span
                    className={`ml-2 font-medium ${
                      robot.return_percentage >= 0
                        ? "text-green-600"
                        : "text-red-600"
                    }`}
                  >
                    {formatPercentage(robot.return_percentage)}
                  </span>
                </div>
              </div>
            </div>

            {/* Performance Bar */}
            <div className="mt-3">
              <div className="flex items-center justify-between text-xs text-gray-600 mb-1">
                <span>Performance</span>
                <span>{formatPercentage(robot.return_percentage)}</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className={`h-2 rounded-full ${
                    robot.return_percentage >= 0 ? "bg-green-500" : "bg-red-500"
                  }`}
                  style={{
                    width: `${Math.min(
                      Math.abs(robot.return_percentage || 0),
                      100
                    )}%`,
                  }}
                ></div>
              </div>
            </div>

            {/* Last Signal */}
            {robot.last_signal_at && (
              <div className="mt-3 text-xs text-gray-500">
                Último sinal:{" "}
                {new Date(robot.last_signal_at).toLocaleString("pt-BR")}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Summary */}
      <div className="mt-6 pt-4 border-t border-gray-200">
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div className="text-center">
            <div className="text-gray-600">Total P&L</div>
            <div
              className={`text-lg font-semibold ${
                robots.reduce(
                  (sum, robot) => sum + (robot.total_pnl || 0),
                  0
                ) >= 0
                  ? "text-green-600"
                  : "text-red-600"
              }`}
            >
              {formatCurrency(
                robots.reduce((sum, robot) => sum + (robot.total_pnl || 0), 0)
              )}
            </div>
          </div>
          <div className="text-center">
            <div className="text-gray-600">Robôs Ativos</div>
            <div className="text-lg font-semibold text-gray-900">
              {robots.filter((robot) => robot.status === "active").length} /{" "}
              {robots.length}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ActiveRobots;
