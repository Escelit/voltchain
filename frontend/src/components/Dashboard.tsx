'use client';

import React from 'react';
import { 
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area 
} from 'recharts';
import { Zap, TrendingUp, Shield, Activity, ArrowUpRight, ArrowDownLeft } from 'lucide-react';

const data = [
  { name: '00:00', generation: 400, consumption: 240 },
  { name: '04:00', generation: 300, consumption: 139 },
  { name: '08:00', generation: 200, consumption: 980 },
  { name: '12:00', generation: 278, consumption: 390 },
  { name: '16:00', generation: 189, consumption: 480 },
  { name: '20:00', generation: 239, consumption: 380 },
  { name: '23:59', generation: 349, consumption: 430 },
];

const trades = [
  { id: 1, type: 'Sell', amount: '15.5 kWh', price: '0.12 XLM', status: 'Completed', time: '2 mins ago' },
  { id: 2, type: 'Buy', amount: '8.2 kWh', price: '0.15 XLM', status: 'Pending', time: '15 mins ago' },
  { id: 3, type: 'Sell', amount: '22.0 kWh', price: '0.11 XLM', status: 'Completed', time: '1 hour ago' },
];

export default function Dashboard() {
  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <header className="flex justify-between items-end">
        <div>
          <h1 className="text-4xl font-bold text-gradient">VoltChain Dashboard</h1>
          <p className="text-slate-400 mt-2">Empowering community energy trading on Stellar</p>
        </div>
        <div className="glass px-6 py-3 rounded-2xl flex items-center gap-4">
          <div className="text-right">
            <p className="text-xs text-slate-500 uppercase tracking-wider">Wallet Balance</p>
            <p className="text-xl font-mono font-bold text-amber-500">1,240.50 XLM</p>
          </div>
          <div className="w-10 h-10 rounded-full bg-amber-500/20 flex items-center justify-center">
            <Shield className="text-amber-500 w-5 h-5" />
          </div>
        </div>
      </header>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        {[
          { label: 'Current Generation', value: '4.2 kW', icon: Zap, color: 'text-amber-500' },
          { label: 'Total Traded', value: '1,280 kWh', icon: TrendingUp, color: 'text-emerald-500' },
          { label: 'Avg. Price', value: '0.13 XLM', icon: Activity, color: 'text-blue-500' },
          { label: 'Green Score', value: '98%', icon: Shield, color: 'text-purple-500' },
        ].map((stat, i) => (
          <div key={i} className="glass p-6 rounded-3xl hover:border-amber-500/30 transition-all duration-300">
            <div className={`w-12 h-12 rounded-2xl bg-white/5 flex items-center justify-center mb-4 ${stat.color}`}>
              <stat.icon size={24} />
            </div>
            <p className="text-slate-400 text-sm">{stat.label}</p>
            <p className="text-2xl font-bold mt-1">{stat.value}</p>
          </div>
        ))}
      </div>

      {/* Main Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 glass p-8 rounded-3xl h-[400px]">
          <h2 className="text-xl font-semibold mb-6 flex items-center gap-2">
            <Activity className="text-amber-500" /> Energy Activity (24h)
          </h2>
          <ResponsiveContainer width="100%" height="85%">
            <AreaChart data={data}>
              <defs>
                <linearGradient id="colorGen" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#f59e0b" stopOpacity={0}/>
                </linearGradient>
                <linearGradient id="colorCons" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" vertical={false} />
              <XAxis dataKey="name" stroke="#ffffff40" fontSize={12} tickLine={false} axisLine={false} />
              <YAxis stroke="#ffffff40" fontSize={12} tickLine={false} axisLine={false} />
              <Tooltip 
                contentStyle={{ background: '#111', border: '1px solid #333', borderRadius: '12px' }}
                itemStyle={{ color: '#fff' }}
              />
              <Area type="monotone" dataKey="generation" stroke="#f59e0b" fillOpacity={1} fill="url(#colorGen)" strokeWidth={3} />
              <Area type="monotone" dataKey="consumption" stroke="#3b82f6" fillOpacity={1} fill="url(#colorCons)" strokeWidth={3} />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="glass p-8 rounded-3xl">
          <h2 className="text-xl font-semibold mb-6">Recent Trades</h2>
          <div className="space-y-6">
            {trades.map((trade) => (
              <div key={trade.id} className="flex items-center justify-between group cursor-pointer">
                <div className="flex items-center gap-4">
                  <div className={`w-10 h-10 rounded-full flex items-center justify-center ${
                    trade.type === 'Sell' ? 'bg-emerald-500/10 text-emerald-500' : 'bg-blue-500/10 text-blue-500'
                  }`}>
                    {trade.type === 'Sell' ? <ArrowUpRight size={20} /> : <ArrowDownLeft size={20} />}
                  </div>
                  <div>
                    <p className="font-medium">{trade.type} {trade.amount}</p>
                    <p className="text-xs text-slate-500">{trade.time}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="font-mono font-bold">{trade.price}</p>
                  <p className={`text-[10px] uppercase tracking-wider ${
                    trade.status === 'Completed' ? 'text-emerald-500' : 'text-amber-500'
                  }`}>{trade.status}</p>
                </div>
              </div>
            ))}
          </div>
          <button className="w-full mt-8 py-3 rounded-2xl bg-amber-500 text-black font-bold hover:bg-amber-400 transition-colors">
            New Trade
          </button>
        </div>
      </div>
    </div>
  );
}
