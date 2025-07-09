import React from "react";
import {
  CurrencyDollarIcon,
  ArrowTrendingUpIcon,
  ArrowTrendingDownIcon,
  ChartBarIcon,
  CogIcon,
} from "@heroicons/react/24/outline";

const StatsCards = ({ stats }) => {
  if (!stats) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {[...Array(4)].map((_, i) => (
          <div key={i} className="card animate-pulse">
            <div className="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
            <div className="h-8 bg-gray-200 rounded w-1/2"></div>
          </div>
        ))}
      </div>
    );
  }

  const formatCurrency = (value) => {
    return new Intl.NumberFormat("pt-BR", {
      style: "currency",
      currency: "USD",
    }).format(value || 0);
  };

  const formatPercentage = (value) => {
    const num = parseFloat(value || 0);
    return `${num >= 0 ? "+" : ""}${num.toFixed(2)}%`;
  };

  const cards = [
    {
      title: "Saldo Total",
      value: formatCurrency(stats.total_balance),
      icon: CurrencyDollarIcon,
      color: "text-blue-600",
      bgColor: "bg-blue-100",
      change: stats.balance_change,
      changeType: stats.balance_change >= 0 ? "positive" : "negative",
    },
    {
      title: "Lucro/Prejuízo",
      value: formatCurrency(stats.total_pnl),
      icon: stats.total_pnl >= 0 ? ArrowTrendingUpIcon : ArrowTrendingDownIcon,
      color: stats.total_pnl >= 0 ? "text-green-600" : "text-red-600",
      bgColor: stats.total_pnl >= 0 ? "bg-green-100" : "bg-red-100",
      change: stats.pnl_change,
      changeType: stats.pnl_change >= 0 ? "positive" : "negative",
    },
    {
      title: "Trades Hoje",
      value: stats.trades_today || 0,
      icon: ChartBarIcon,
      color: "text-purple-600",
      bgColor: "bg-purple-100",
      change: stats.trades_change,
      changeType: stats.trades_change >= 0 ? "positive" : "negative",
      isNumber: true,
    },
    {
      title: "Robôs Ativos",
      value: stats.active_robots || 0,
      icon: CogIcon,
      color: "text-orange-600",
      bgColor: "bg-orange-100",
      change: stats.robots_change,
      changeType: stats.robots_change >= 0 ? "positive" : "negative",
      isNumber: true,
    },
  ];

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {cards.map((card, index) => (
        <div key={index} className="card">
          <div className="flex items-center">
            <div className={`p-3 rounded-lg ${card.bgColor}`}>
              <card.icon className={`h-6 w-6 ${card.color}`} />
            </div>
            <div className="ml-4 flex-1">
              <p className="text-sm font-medium text-gray-600">{card.title}</p>
              <p className="text-2xl font-semibold text-gray-900">
                {card.value}
              </p>
            </div>
          </div>

          {card.change !== undefined && (
            <div className="mt-4 flex items-center">
              <div
                className={`flex items-center text-sm ${
                  card.changeType === "positive"
                    ? "text-green-600"
                    : "text-red-600"
                }`}
              >
                {card.changeType === "positive" ? (
                  <ArrowTrendingUpIcon className="h-4 w-4 mr-1" />
                ) : (
                  <ArrowTrendingDownIcon className="h-4 w-4 mr-1" />
                )}
                {card.isNumber ? card.change : formatPercentage(card.change)}
              </div>
              <span className="text-sm text-gray-500 ml-2">
                vs. período anterior
              </span>
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default StatsCards;
