import { ButtonHTMLAttributes } from "react";

type Variant = "primary" | "secondary" | "danger" | "ghost";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
}

const styles: Record<Variant, string> = {
  primary: "bg-slate-900 text-white hover:bg-slate-800 disabled:bg-slate-400",
  secondary: "bg-white text-slate-900 border border-slate-300 hover:bg-slate-50",
  danger: "bg-red-600 text-white hover:bg-red-700",
  ghost: "text-slate-600 hover:bg-slate-100",
};

export default function Button({ variant = "primary", className = "", ...rest }: Props) {
  return (
    <button
      className={`px-4 py-2 rounded text-sm font-medium transition ${styles[variant]} ${className}`}
      {...rest}
    />
  );
}
