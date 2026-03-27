import React from "react";
import { Link, useLocation } from "react-router-dom";
import { LayoutDashboard, Shield, Activity } from "lucide-react";

const navItems = [
  { label: "Dashboard", path: "/", icon: LayoutDashboard },
  { label: "Rules", path: "/rules", icon: Shield },
  { label: "Checker", path: "/checker", icon: Activity },
];

const Navbar = () => {
  const location = useLocation();

  return (
    <div className="bg-gray-900 text-white">
      <div className="max-w-6xl mx-auto flex justify-between items-center p-4">
        
        <h1 className="text-xl font-bold">Rate Limiter</h1>

        <div className="flex gap-6">
          {navItems.map((item) => {
            const IconComponent = item.icon;

            return (
              <Link
                key={item.path}
                to={item.path}
                className={`flex items-center gap-2 px-3 py-2 rounded-md ${
                  location.pathname === item.path
                    ? "bg-gray-700"
                    : "hover:bg-gray-800"
                }`}
              >
                <IconComponent size={18} />
                <span>{item.label}</span>
              </Link>
            );
          })}
        </div>

      </div>
    </div>
  );
};

export default Navbar;