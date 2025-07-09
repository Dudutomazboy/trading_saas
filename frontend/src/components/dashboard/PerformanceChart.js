import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Area,
  AreaChart,
} from "recharts";
import { format, parseISO } from "date-fns";
import { ptBR } from "date-fns/locale";

const PerformanceChart = ({ data, period }) => {
  if (!data || data.length === 0) {
    return (
      <div className="card">
        <h3 className="text-lg font-medium text-gray-900 mb-4">
          Performance do Portfólio
        </h3>
        <div className="h-80 flex items-center justify-center">
          <div className="text-gray-500">Nenhum dado disponível</div>
        </div>
      </div>
    );
  }

  const formatDate = (dateString) => {
    try {
      const date = parseISO(dateString);
      switch (period) {
        case "1d":
          return format(date, "HH:mm", { locale: ptBR });
        case "7d":
          return format(date, "dd/MM", { locale: ptBR });
        case "30d":
        case "90d":
          return format(date, "dd/MM", { locale: ptBR });
        default:
          return format(date, "dd/MM", { locale: ptBR });
      }
    } catch (error) {
      return dateString;
    }
  };

  const formatCurrency = (value) => {
    return new Intl.NumberFormat("pt-BR", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 2,
    }).format(value);
  };

  const formatPercentage = (value) => {
    return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`;
  };

  const CustomTooltip = ({ active, payload, label }) => {
    if (active && payload && payload.length) {
      const data = payload[0].payload;
      return (
        <div className="bg-white p-4 border border-gray-200 rounded-lg shadow-lg">
          <p className="text-sm text-gray-600 mb-2">{formatDate(label)}</p>
          <div className="space-y-1">
            <p className="text-sm">
              <span className="font-medium">Saldo:</span>{" "}
              <span className="text-blue-600">
                {formatCurrency(data.balance)}
              </span>
            </p>
            <p className="text-sm">
              <span className="font-medium">P&L:</span>{" "}
              <span
                className={data.pnl >= 0 ? "text-green-600" : "text-red-600"}
              >
                {formatCurrency(data.pnl)}
              </span>
            </p>
            <p className="text-sm">
              <span className="font-medium">Retorno:</span>{" "}
              <span
                className={
                  data.return_percentage >= 0
                    ? "text-green-600"
                    : "text-red-600"
                }
              >
                {formatPercentage(data.return_percentage)}
              </span>
            </p>
          </div>
        </div>
      );
    }
    return null;
  };

  // Calculate overall performance
  const firstValue = data[0]?.balance || 0;
  const lastValue = data[data.length - 1]?.balance || 0;
  const totalReturn = ((lastValue - firstValue) / firstValue) * 100;
  const totalPnL = lastValue - firstValue;

  return (
    <div className="card">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-lg font-medium text-gray-900">
            Performance do Portfólio
          </h3>
          <p className="text-sm text-gray-600">
            Evolução do saldo e retorno no período
          </p>
        </div>
        <div className="text-right">
          <div className="text-sm text-gray-600">Retorno Total</div>
          <div
            className={`text-lg font-semibold ${
              totalReturn >= 0 ? "text-green-600" : "text-red-600"
            }`}
          >
            {formatPercentage(totalReturn)}
          </div>
          <div
            className={`text-sm ${
              totalPnL >= 0 ? "text-green-600" : "text-red-600"
            }`}
          >
            {formatCurrency(totalPnL)}
          </div>
        </div>
      </div>

      <div className="h-80">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart
            data={data}
            margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
          >
            <defs>
              <linearGradient id="balanceGradient" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#3B82F6" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#3B82F6" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#E5E7EB" />
            <XAxis
              dataKey="timestamp"
              tickFormatter={formatDate}
              stroke="#6B7280"
              fontSize={12}
            />
            <YAxis
              tickFormatter={(value) => formatCurrency(value)}
              stroke="#6B7280"
              fontSize={12}
            />
            <Tooltip content={<CustomTooltip />} />
            <Area
              type="monotone"
              dataKey="balance"
              stroke="#3B82F6"
              strokeWidth={2}
              fill="url(#balanceGradient)"
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {/* Performance Metrics */}
      <div className="mt-6 grid grid-cols-3 gap-4 pt-4 border-t border-gray-200">
        <div className="text-center">
          <div className="text-sm text-gray-600">Maior Alta</div>
          <div className="text-lg font-semibold text-green-600">
            {formatCurrency(Math.max(...data.map((d) => d.balance)))}
          </div>
        </div>
        <div className="text-center">
          <div className="text-sm text-gray-600">Maior Baixa</div>
          <div className="text-lg font-semibold text-red-600">
            {formatCurrency(Math.min(...data.map((d) => d.balance)))}
          </div>
        </div>
        <div className="text-center">
          <div className="text-sm text-gray-600">Volatilidade</div>
          <div className="text-lg font-semibold text-gray-900">
            {(() => {
              const returns = data.map((d) => d.return_percentage || 0);
              const avg = returns.reduce((a, b) => a + b, 0) / returns.length;
              const variance =
                returns.reduce((a, b) => a + Math.pow(b - avg, 2), 0) /
                returns.length;
              const volatility = Math.sqrt(variance);
              return `${volatility.toFixed(2)}%`;
            })()}
          </div>
        </div>
      </div>
    </div>
  );
};

export default PerformanceChart;
