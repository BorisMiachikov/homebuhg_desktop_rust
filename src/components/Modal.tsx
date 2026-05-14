import { ReactNode } from "react";

interface Props {
  open: boolean;
  title: string;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
  width?: string;
}

export default function Modal({ open, title, onClose, children, footer, width = "max-w-lg" }: Props) {
  if (!open) return null;
  return (
    <div
      className="fixed inset-0 bg-slate-900/40 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className={`bg-white rounded-lg shadow-xl ${width} w-full max-h-[90vh] flex flex-col`}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="px-6 py-4 border-b border-slate-200 flex items-center justify-between">
          <h2 className="text-lg font-semibold text-slate-900">{title}</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-700 text-2xl leading-none"
          >
            ×
          </button>
        </div>
        <div className="px-6 py-4 overflow-auto flex-1">{children}</div>
        {footer && (
          <div className="px-6 py-3 border-t border-slate-200 bg-slate-50 flex justify-end gap-2 rounded-b-lg">
            {footer}
          </div>
        )}
      </div>
    </div>
  );
}
