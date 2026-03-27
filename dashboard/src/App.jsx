import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import Navbar from "./components/Navbar";
import Dashboard from "./pages/Dashboard";
import Rules from "./pages/Rules";
import Checker from "./pages/Checker";

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <div style={{ minHeight: "100vh", background: "#0f172a" }}>
          <Navbar />
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/rules" element={<Rules />} />
            <Route path="/checker" element={<Checker />} />
          </Routes>
        </div>
      </BrowserRouter>
    </QueryClientProvider>
  );
}