// src/components/Charts.jsx
import React from "react";
import {
	LineChart as RechartsLineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	Legend,
	ResponsiveContainer,
} from "recharts";

export const LineChart = ({ data }) => {
	return (
		<ResponsiveContainer width="100%" height="100%">
			<RechartsLineChart
				data={data}
				margin={{
					top: 5,
					right: 30,
					left: 20,
					bottom: 5,
				}}>
				<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
				<XAxis
					dataKey="month"
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
				/>
				<YAxis
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
					tickFormatter={(value) => `${value} USDC`}
				/>
				<Tooltip
					contentStyle={{
						backgroundColor: "#fff",
						border: "1px solid #e5e7eb",
						borderRadius: "6px",
						boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
					}}
					formatter={(value) => [`${value} USDC`]}
				/>
				<Legend />
				<Line
					type="monotone"
					dataKey="budget"
					stroke="#3b82f6"
					strokeWidth={2}
					dot={{
						stroke: "#3b82f6",
						strokeWidth: 2,
						r: 4,
						fill: "#fff",
					}}
					activeDot={{
						r: 6,
						stroke: "#3b82f6",
						strokeWidth: 2,
						fill: "#fff",
					}}
				/>
				<Line
					type="monotone"
					dataKey="spent"
					stroke="#10b981"
					strokeWidth={2}
					dot={{
						stroke: "#10b981",
						strokeWidth: 2,
						r: 4,
						fill: "#fff",
					}}
					activeDot={{
						r: 6,
						stroke: "#10b981",
						strokeWidth: 2,
						fill: "#fff",
					}}
				/>
			</RechartsLineChart>
		</ResponsiveContainer>
	);
};

// Bar Chart Component
export const BarChart = ({ data }) => {
	return (
		<ResponsiveContainer width="100%" height="100%">
			<RechartsBarChart
				data={data}
				margin={{
					top: 5,
					right: 30,
					left: 20,
					bottom: 5,
				}}>
				<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
				<XAxis
					dataKey="name"
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
				/>
				<YAxis
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
					tickFormatter={(value) => `${value} USDC`}
				/>
				<Tooltip
					contentStyle={{
						backgroundColor: "#fff",
						border: "1px solid #e5e7eb",
						borderRadius: "6px",
						boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
					}}
					formatter={(value) => [`${value} USDC`]}
				/>
				<Legend />
				<Bar dataKey="value" fill="#3b82f6" radius={[4, 4, 0, 0]} />
			</RechartsBarChart>
		</ResponsiveContainer>
	);
};

// Area Chart Component
export const AreaChart = ({ data }) => {
	return (
		<ResponsiveContainer width="100%" height="100%">
			<RechartsAreaChart
				data={data}
				margin={{
					top: 5,
					right: 30,
					left: 20,
					bottom: 5,
				}}>
				<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
				<XAxis
					dataKey="name"
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
				/>
				<YAxis
					stroke="#6b7280"
					fontSize={12}
					tickLine={false}
					tickFormatter={(value) => `${value} USDC`}
				/>
				<Tooltip
					contentStyle={{
						backgroundColor: "#fff",
						border: "1px solid #e5e7eb",
						borderRadius: "6px",
						boxShadow: "0 2px 4px rgba(0,0,0,0.1)",
					}}
					formatter={(value) => [`${value} USDC`]}
				/>
				<Legend />
				<Area
					type="monotone"
					dataKey="value"
					stroke="#3b82f6"
					fill="#93c5fd"
					fillOpacity={0.3}
				/>
			</RechartsAreaChart>
		</ResponsiveContainer>
	);
};

export default {
	LineChart,
	BarChart,
	AreaChart,
};
